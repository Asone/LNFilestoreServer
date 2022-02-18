use diesel::result::Error;
use rocket::{
    http::Status,
    request::{self, local_cache, FromRequest, Outcome},
    Request, State,
};

use tonic_lnd::rpc::invoice::InvoiceState;

use crate::{
    db::{
        models::api_payment::{ApiPayment, NewApiPayment},
        PostgresConn,
    },
    lnd::{
        client::LndClient,
        invoice::{InvoiceParams, InvoiceUtils},
    },
};
/**
   Provides a payment request validation guard.
   If no payment_request is provided through the
*/
pub struct PaymentRequestHeader(pub Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PaymentRequestHeader {
    type Error = Option<&'r str>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<PostgresConn>().await.succeeded();
        match pool {
            Some(conn) => {
                let payment_request_header = request.headers().get_one("payment_request").clone();
                match payment_request_header {
                    Some(payment_request) => {
                        let api_payment = conn.find_api_payment(payment_request.to_string()).await;
                        match api_payment {
                            Some(payment) => {
                                outcome_from_payment_request(request, payment_request).await
                            }
                            None => Outcome::Failure((Status::PaymentRequired, None)),
                        }
                    }
                    None => {
                        let lnd_client_result = request.guard::<LndClient>().await.succeeded();
                        match lnd_client_result {
                            Some(lnd_client) => {
                                request
                                    .local_cache_async(async {
                                        request_new_api_payment(lnd_client, conn).await
                                    })
                                    .await;
                                Outcome::Failure((Status::PaymentRequired, None))
                            }
                            None => Outcome::Failure((Status::InternalServerError, None)),
                        }
                    }
                }
            }
            None => Outcome::Failure((Status::InternalServerError, None)),
        }
    }
}

async fn request_new_api_payment(
    lnd_client: LndClient,
    db: PostgresConn,
) -> Result<ApiPayment, Error> {
    let client = lnd_client.0;
    let invoice =
        InvoiceUtils::generate_invoice(client, InvoiceParams::new(None, None, None)).await;

    db.run(move |c| ApiPayment::create(NewApiPayment::from(invoice), c))
        .await
}

async fn outcome_from_payment_request<'r>(
    request: &'r Request<'_>,
    payment_request: &'r str,
) -> Outcome<PaymentRequestHeader, Option<&'r str>> {
    let lnd_client = request.guard::<LndClient>().await.succeeded();
    match lnd_client {
        Some(client) => {
            let invoice_state = client.get_invoice_status(payment_request.to_string()).await;
            match invoice_state {
                Ok(state) => outcome_from_invoice_state(state, payment_request),
                Err(_) => Outcome::Failure((Status::InternalServerError, None)),
            }
        }
        None => Outcome::Failure((Status::InternalServerError, None)),
    }
}

fn outcome_from_invoice_state(
    state: InvoiceState,
    payment_request: &str,
) -> Outcome<PaymentRequestHeader, Option<&str>> {
    match state {
        InvoiceState::Settled => {
            Outcome::Success(PaymentRequestHeader(Some(payment_request.to_string())))
        }
        InvoiceState::Open => Outcome::Failure((Status::PaymentRequired, None)),
        InvoiceState::Accepted => Outcome::Failure((Status::Processing, None)),
        InvoiceState::Canceled => Outcome::Failure((Status::PaymentRequired, None)),
    }
}
