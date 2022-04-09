// use jsonwebtoken::{DecodingKey, Validation, Algorithm, TokenData};
// use rocket::{ http::Status, Request, data::Outcome, request::FromRequest};

// pub struct UserGuard(pub bool);

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for UserGuard {
//     type Error = ();

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//             let authorization = request.headers().get_one("Authorization");

//             match authorization {
//                 Some(authorization) => {
//                     match jsonwebtoken::decode(authorization, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)){
//                         Ok(token) => {
//                             Outcome::Forward(UserGuard(true))
//                         },
//                         Err(_) => {}
//                     }
//                 },
//                 None => {}
//             }

//     }
// }

use std::env;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use tonic_lnd::rpc::invoice::InvoiceState;

use crate::db::models::user_token::UserToken;

extern crate dotenv;

pub struct UserGuard(pub bool);

impl UserGuard {
    /// Checks that the authorization header includes Bearer mention
    /// Returns the token without the bearer prefix
    pub fn format_bearer(authorization: &str) -> String {
        let re = regex::Regex::new("^[bB]earer ").unwrap();
        re.replace(authorization, "").to_string()
    }
}

/// Checks the JWT provided and checks if it is valid
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");

        match authorization {
            Some(authorization) => {
                let formated_token = Self::format_bearer(authorization);
                let token = jsonwebtoken::decode::<UserToken>(
                    formated_token.as_str(),
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::new(Algorithm::HS256),
                );

                match token {
                    Ok(token) => {
                        let t = token;
                        Outcome::Success(UserGuard(true))
                    }
                    Err(_) => Outcome::Failure((Status::ExpectationFailed, ())),
                }
            }
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
