use std::path::Path;

use chrono::Utc;
use rocket::{
    fs::NamedFile,
    http::{Header, Status},
    response::{content::RawJson, status},
};
use tonic::{codegen::InterceptedService, transport::Channel};
use tonic_lnd::{
    rpc::{invoice::InvoiceState, lightning_client::LightningClient, Invoice},
    MacaroonInterceptor,
};
use uuid::Uuid;

use crate::{
    db::{
        models::{
            media::Media,
            media_payment::{MediaPayment, NewMediaPayment},
        },
        PostgresConn,
    },
    lnd::{
        client::LndClient,
        invoice::{InvoiceParams, InvoiceUtils},
    },
    responders::download::DownloadResponder,
};

#[derive(Debug)]
pub enum FileHandlingError {
    MediaNotFound,
    InvoiceNotFound,
    DbFailure,
    LNFailure,
    UuidParsingError,
    PaymentRequired,
}

/// A route to retrieve files behind the paywall.
#[rocket::get("/file/<uuid>?<invoice>")]
pub async fn get_file(
    uuid: String,
    invoice: Option<String>,
    db: PostgresConn,
    lnd: LndClient,
) -> Result<DownloadResponder, status::Custom<Option<RawJson<String>>>> {
    // Calls the get_media to try to retrieve the requested media from database
    let media = match get_media(&uuid, &db).await {
        Ok(media) => media,
        Err(e) => match e {
            FileHandlingError::DbFailure => {
                return Err(status::Custom(Status::InternalServerError, None))
            }
            FileHandlingError::MediaNotFound => return Err(status::Custom(Status::NotFound, None)),
            FileHandlingError::UuidParsingError => {
                return Err(status::Custom(Status::BadRequest, None))
            }
            _ => return Err(status::Custom(Status::ImATeapot, None)),
        },
    };

    // If the media exists and is free we should deliver it to the user without performing any further operation
    if media.price == 0 {
        return set_download_responder(media).await;
    }

    // Otherwise we ensure try to retrieve an associated payment to the requested media.
    // see get_media_payment for handling process
    let payment = match get_media_payment(invoice, &media.uuid, &db).await {
        Ok(payment) => payment,
        Err(e) => match e {
            FileHandlingError::DbFailure => {
                return Err(status::Custom(Status::InternalServerError, None))
            }
            FileHandlingError::PaymentRequired => {
                match request_new_media_payment(&media, lnd, db).await {
                    Ok(invoice) => {
                        let data = format!("{{ payment_request: {}}}", invoice.request);
                        return Err(status::Custom(Status::PaymentRequired, Some(RawJson(data))));
                    }
                    Err(e) => match e {
                        FileHandlingError::DbFailure => {
                            return Err(status::Custom(Status::InternalServerError, None))
                        }
                        _ => return Err(status::Custom(Status::ImATeapot, None)),
                    },
                }
            }
            _ => return Err(status::Custom(Status::ImATeapot, None)),
        },
    };

    let invoice = get_invoice(payment.clone(), &lnd.0).await; // .map_err(|error| return error).unwrap();

    if invoice.is_err() {
        return match invoice.unwrap_err() {
            FileHandlingError::InvoiceNotFound => Err(status::Custom(Status::NotFound, None)),
            FileHandlingError::LNFailure => Err(status::Custom(Status::InternalServerError, None)),
            _ => Err(status::Custom(Status::ImATeapot, None)),
        };
    }

    let invoice = invoice.unwrap();

    match invoice.state() {
        InvoiceState::Settled => match payment.clone().valid_until {
            Some(valid_until) => match valid_until >= Utc::now().naive_utc() {
                true => set_download_responder(media).await,
                false => match request_new_media_payment(&media, lnd, db).await {
                    Ok(invoice) => {
                        let data = format!("{{ payment_request: {}}}", invoice.request);

                        Err(status::Custom(Status::PaymentRequired, Some(RawJson(data))))
                    }
                    Err(e) => match e {
                        FileHandlingError::DbFailure => {
                            Err(status::Custom(Status::InternalServerError, None))
                        }
                        _ => Err(status::Custom(Status::ImATeapot, None)),
                    },
                },
            },
            None => set_download_responder(media).await,
        },
        InvoiceState::Accepted => Err(status::Custom(Status::NotFound, None)),
        InvoiceState::Canceled => {
            let invoice = request_new_media_payment(&media, lnd, db).await;
            match invoice {
                Ok(invoice) => {
                    let data = format!("{{ payment_request: {}}}", invoice.request);

                    Err(status::Custom(Status::PaymentRequired, Some(RawJson(data))))
                }
                Err(e) => match e {
                    FileHandlingError::DbFailure => {
                        Err(status::Custom(Status::InternalServerError, None))
                    }
                    _ => Err(status::Custom(Status::ImATeapot, None)),
                },
            }
        }
        InvoiceState::Open => {
            let data = format!("{{ payment_request: {}}}", invoice.payment_request);

            Err(status::Custom(Status::PaymentRequired, Some(RawJson(data))))
        }
    }
}

/// Generates an invoice and saves its value in databasee
async fn request_new_media_payment(
    media: &Media,
    lnd_client: LndClient,
    db: PostgresConn,
) -> Result<MediaPayment, FileHandlingError> {
    // Loads the client
    let client = lnd_client.0;

    // let uuid = Uuid::parse_str(uuid.as_str());

    // Return error if uuid parsing fails.
    // if uuid.is_err() {
    //     return Err(FileHandlingError::UuidParsingError);
    // }

    let uuid = media.uuid.to_owned();

    // Calls utility to generate an invoice/
    // Todo : Generate
    let invoice = InvoiceUtils::generate_invoice(
        client,
        InvoiceParams::new(Some(media.price.into()), None, None),
    )
    .await;

    let media_payment = db
        .run(move |c| MediaPayment::create(NewMediaPayment::from((invoice, uuid)), c))
        .await;

    match media_payment {
        Ok(media_payment) => Ok(media_payment),
        Err(_) => Err(FileHandlingError::DbFailure),
    }
}

// Retrieves media from database
async fn get_media(uuid: &String, db: &PostgresConn) -> Result<Media, FileHandlingError> {
    let uuid = Uuid::parse_str(uuid.as_str());

    match uuid {
        Ok(uuid) => {
            let media = db.run(move |c| Media::find_one_by_uuid(uuid, c)).await;
            match media {
                Ok(media) => match media {
                    Some(media) => Ok(media),
                    None => Err(FileHandlingError::MediaNotFound),
                },
                Err(_) => Err(FileHandlingError::DbFailure),
            }
        }
        Err(_) => Err(FileHandlingError::UuidParsingError),
    }
}

// Retrieves a media payment based on
async fn get_media_payment(
    payment_request: Option<String>,
    media_uuid: &Uuid,
    db: &PostgresConn,
) -> Result<MediaPayment, FileHandlingError> {
    match payment_request {
        // Ensure there is some payment_request provided
        Some(payment_request) => {
            // Retrieve recorded payment request from db
            let payment = db
                .run(move |c| MediaPayment::find_one_by_request(payment_request, c))
                .await;
            match payment {
                Ok(payment) => {
                    match payment {
                        Some(payment) => {
                            // Ensure the retrieved payment request matched the requested file association
                            match &payment.media_uuid == media_uuid {
                                true => Ok(payment),
                                false => Err(FileHandlingError::MediaNotFound),
                            }
                        }
                        None => Err(FileHandlingError::PaymentRequired),
                    }
                }
                Err(_) => Err(FileHandlingError::DbFailure),
            }
        }
        None => Err(FileHandlingError::PaymentRequired),
    }
}

async fn get_invoice(
    media_payment: MediaPayment,
    lnd: &LightningClient<InterceptedService<Channel, MacaroonInterceptor>>,
) -> Result<Invoice, FileHandlingError> {
    match InvoiceUtils::get_invoice_state_from_payment_request(lnd, media_payment.request).await {
        Ok(invoice) => match invoice {
            Some(invoice) => Ok(invoice),
            None => Err(FileHandlingError::InvoiceNotFound),
        },
        Err(_) => Err(FileHandlingError::LNFailure),
    }
}

async fn set_download_responder(
    media: Media,
) -> Result<DownloadResponder, status::Custom<Option<RawJson<String>>>> {
    let path = Path::new(&media.absolute_path);
    let filename = path.file_name();

    match filename {
        Some(filename) => {
            let disposition_value =
                format!(r#"attachment; filename="{}""#, filename.to_str().unwrap());
            Ok(DownloadResponder {
                inner: NamedFile::open(path).await.unwrap(),
                disposition: Header::new("Content-Disposition", disposition_value),
            })
        }
        None => Err(status::Custom(Status::InternalServerError, None)),
    }
}
