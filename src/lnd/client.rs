

use std::env::{self};

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

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
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let cert_file = env::var("LND_CERTFILE_PATH");

        if cert_file.is_err() {
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let macaroon_file = env::var("LND_MACAROON_PATH");

        if macaroon_file.is_err() {
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }

        let client =
            tonic_lnd::connect(address.unwrap(), cert_file.unwrap(), macaroon_file.unwrap()).await;

        match client {
            Ok(result) => Outcome::Success(LndClient(result)),
            Err(_e) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}