use std::env;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use tonic_lnd::rpc::invoice::InvoiceState;

use super::invoice::InvoiceUtils;
extern crate dotenv;

pub struct LndClient(pub tonic_lnd::Client);

/*
The below implementation allows us to start the Lnd client instance that will
be later used in a request process by being injected in context object
*/
#[rocket::async_trait]
impl<'r> FromRequest<'r> for LndClient {
    type Error = ();

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let address = env::var("LND_ADDRESS");

        if address.is_err() {
            println!("missing address for lnd");
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let address = address.unwrap();

        let cert_file = env::var("LND_CERTFILE_PATH");

        if cert_file.is_err() {
            println!("cert failure for lnd");
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let macaroon_file = env::var("LND_MACAROON_PATH");

        if macaroon_file.is_err() {
            println!("macaroon failure");
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let client = tonic_lnd::connect(address, cert_file.unwrap(), macaroon_file.unwrap()).await;

        match client {
            Ok(result) => Outcome::Success(LndClient(result)),
            Err(e) => {
                println!("Error while connecting to LND server");
                println!("{}", e);
                Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}

impl LndClient {
    pub async fn get_invoice_status(
        self,
        payment_hash: String,
    ) -> Result<InvoiceState, &'static str> {
        match InvoiceUtils::get_invoice_state_from_payment_request(&self.0, payment_hash).await {
            Ok(invoice_result) => match invoice_result {
                Some(invoice) => Ok(invoice.state()),
                None => Err("No invoice found"),
            },
            // LND Server says there's no invoice matching
            // Invoice is broken. Maybe we should serve a new invoice here ?
            Err(_) => Err("Error fetching invoice"),
        }
    }
}
