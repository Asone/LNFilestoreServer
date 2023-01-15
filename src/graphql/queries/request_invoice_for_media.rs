use crate::db::models::media::Media;
use crate::db::models::media::MediaModelType;
use crate::db::models::media_payment::MediaPayment;
use crate::db::models::media_payment::NewMediaPayment;
use crate::db::PostgresConn;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::invoices::CustomInvoiceStateFlag;
use crate::graphql::types::output::invoices::MediaInvoice;
use crate::lnd::invoice::InvoiceParams;
use crate::lnd::invoice::InvoiceUtils;
use juniper::{FieldError, Value};
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic_lnd::rpc::invoice::InvoiceState;
use tonic_lnd::MacaroonInterceptor;

use tonic_lnd::rpc::lightning_client::LightningClient;

/// Requests an invoice and/or its state for a media.
/// The request can get an optional `payment_request`
/// that if provided will have its validity checked.
pub async fn request_invoice_for_media<'a>(
    context: &'a GQLContext,
    uuid: uuid::Uuid,
    payment_request: Option<String>,
) -> Result<MediaInvoice, FieldError> {
    let connection = context.get_db_connection();
    let client = context.get_lnd_client();

    // Get media from db
    let db_result = connection
        .run(move |c| Media::find_one_by_uuid(uuid, c))
        .await;

    // db failure
    if db_result.is_err() {
        return Err(FieldError::new(
            "Error while requesting database",
            Value::Null,
        ));
    }

    // unwrap result if no db failure
    let media = db_result.unwrap();

    // Return error if there is no media found in db
    if media.is_none() {
        return Err(FieldError::new(
            "No media found with provided uuid",
            Value::Null,
        ));
    }

    // unwrap result to get the media. At this point we are sure
    // media is some due to if statement above
    let media = media.unwrap();

    // Dispatch action based on presence of payment_request in request input
    match payment_request {
        Some(payment_request) => {
            check_provided_payment_request(connection, client.clone(), media, payment_request).await
        }
        None => create_media_invoice(connection, client, media).await,
    }
}

async fn create_media_invoice(
    connection: &PostgresConn,
    lnd: &LightningClient<InterceptedService<Channel, MacaroonInterceptor>>,
    media: Media,
) -> Result<MediaInvoice, FieldError> {
    let payment = generate_media_payment(connection, lnd, media).await;
    match payment {
        Ok(payment) => Ok(MediaInvoice::from((payment, InvoiceState::Open))),
        Err(_) => Err(FieldError::new(
            "Error while registering payment request.",
            Value::Null,
        )),
    }
}

/// Processes a check of an invoice state when payment_request input field is provided
async fn check_provided_payment_request(
    connection: &PostgresConn,
    lnd: LightningClient<InterceptedService<Channel, MacaroonInterceptor>>,
    media: Media,
    payment_request: String,
) -> Result<MediaInvoice, FieldError> {
    // Request db to find payment
    let payment = match connection
        .run(move |c| MediaPayment::find_one_by_request(payment_request, c))
        .await
    {
        Ok(payment) => match payment {
            Some(payment) => payment,
            None => {
                return Err(FieldError::new(
                    "No payment found with the provided payment_request",
                    Value::Null,
                ))
            }
        },
        Err(e) => {
            return Err(FieldError::new(
                "Error while requesting database",
                Value::Null,
            ))
        }
    };

    // No matter any other condition, if the payment validity is considered as expired
    // we shall return a new invoice
    if payment.is_expired() {
        let media_for_payment = media.clone();
        match generate_media_payment(connection, &lnd, media_for_payment).await {
            Ok(r) => {
                return Ok(MediaInvoice::from((
                    r,
                    CustomInvoiceStateFlag::ExpiredInvoice,
                )));
            }
            Err(e) => {
                return Err(FieldError::new(
                    "Error while requesting lightning network registry",
                    Value::Null,
                ));
            }
        };
    }

    // Ensure the request media is the same that is associated in the payment
    if payment.media_uuid != media.uuid {
        return Err(FieldError::new(
            "payment_request does not match with the request media",
            Value::Null,
        ));
    }

    let payment_request = payment.request.clone();

    let invoice = match InvoiceUtils::get_invoice_state_from_payment_request(&lnd, payment_request)
        .await
    {
        Ok(r) => match r {
            Some(invoice) => invoice,
            None => {
                return Err(FieldError::new(
                     "No invoice found with the current payment request on the lightning network service",
         Value::Null,
                    ));
            }
        },
        Err(e) => {
            return Err(FieldError::new(
                "Error while requesting lightning network registry",
                Value::Null,
            ));
        }
    };

    // Return result based on the invoice state
    match invoice.state() {
        InvoiceState::Accepted => {
            // If invoice is in accepted state we return the current media invoice
            // with the current state.
            Ok(MediaInvoice::from((payment, invoice.state())))
        }
        InvoiceState::Canceled => {
            // If invoice has been canceled we generate a new one
            let payment = generate_media_payment(connection, &lnd, media).await;

            // We catch the result and return the new media payment.
            // We provide invoice state for previous invoice as
            // this will help returning the replacementpayment output type
            match payment {
                Ok(payment) => Ok(MediaInvoice::from((payment, InvoiceState::Canceled))),
                Err(error) => Err(error),
            }
        }
        InvoiceState::Open => Ok(MediaInvoice::from((payment, invoice.state()))),
        InvoiceState::Settled => Ok(MediaInvoice::from((payment, invoice.state()))),
    }
}

/// Method to generate a media payment
/// With invoice registering on LND
async fn generate_media_payment(
    connection: &PostgresConn,
    lnd: &LightningClient<InterceptedService<Channel, MacaroonInterceptor>>,
    media: Media,
) -> Result<MediaPayment, FieldError> {
    let memo = format!("Buy file \"{}\" with uuid: {}", media.title, media.uuid);
    let params = InvoiceParams::new(Some(media.price as i64), Some(memo), None);
    let invoice = InvoiceUtils::generate_invoice(lnd.clone(), params).await;
    let payment = connection
        .run(move |c| {
            MediaPayment::create(
                NewMediaPayment::from((invoice, media.uuid, MediaModelType::Media(media))),
                c,
            )
        })
        .await;

    match payment {
        Ok(payment) => Ok(payment),
        Err(_) => Err(FieldError::new(
            "Error while registering payment request.",
            Value::Null,
        )),
    }
}

/// Method to generate a field error
/// from a media payment
/// Shall be deleted as not used anymore
fn _field_error_from_media_payment(
    media_payment: MediaPayment,
    message: Option<String>,
) -> FieldError {
    let message = message.unwrap_or("Payment required".to_string());
    let request = media_payment.request.as_str();
    let expires_at = media_payment.expires_at.timestamp() as i32;
    let state = media_payment.state;
    FieldError::new(
        message,
        graphql_value!({
            "state": state,
            "paymentRequest": request,
            "expiresAt": expires_at
        }),
    )
}
