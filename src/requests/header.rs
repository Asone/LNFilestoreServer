use std::env::{self};

use std::convert::Infallible;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

pub struct PaymentRequestHeader(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PaymentRequestHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let payment_request = request.headers().get_one("payment_request");
        match payment_request {
            Some(payment_request) => {
                // check validity
                Outcome::Success(PaymentRequestHeader(payment_request.to_string()))
            }
            // token does not exist
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
