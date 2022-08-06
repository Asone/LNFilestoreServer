use std::env;

extern crate dotenv;

use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, Validation};
use rocket::{
    http::{Cookie, CookieJar, Status},
    request::{FromRequest, Outcome},
    Request,
};

use crate::db::{
    models::{session::UserSession, user::User, user_token::UserToken},
    PostgresConn,
};

/// Builds user session based on jwt auth
pub struct UserGuard(pub Option<User>);

impl UserGuard {
    /// Checks that the authorization header includes Bearer mention
    /// Returns the token without the bearer prefix
    pub fn format_bearer(authorization: &str) -> String {
        let re = regex::Regex::new("^[bB]earer ").unwrap();
        re.replace(authorization, "").to_string()
    }

    /// Retrieves the secret env
    fn get_secret() -> Result<String, ()> {
        let secret = env::var("JWT_TOKEN_SECRET");

        match secret {
            Ok(secret) => Ok(secret),
            Err(_) => Err(()),
        }
    }

    // fn session_regeneration() -> {

    // }
}

/// Checks the JWT provided and checks if it is valid
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");
        let session = request.cookies().get("session");
        match session {
            Some(session) => {
                // let formated_token = Self::format_bearer(authorization);
                let secret = Self::get_secret().unwrap();

                let token = jsonwebtoken::decode::<UserToken>(
                    session.value(),
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS256),
                );

                let pool = request.guard::<PostgresConn>().await.succeeded();

                match token {
                    Ok(token) => {
                        let user_uuid = uuid::Uuid::parse_str(token.claims.user.as_str()).unwrap();
                        let session_uuid = token.claims.uuid;

                        match pool {
                            Some(conn) => {
                                let cookies = request.guard::<&'r CookieJar>().await;

                                match cookies {
                                    Outcome::Success(cookies) => {
                                        let session = conn
                                            .run(move |c| {
                                                UserSession::update_session_expiry(session_uuid, c)
                                            })
                                            .await;

                                        match session {
                                            Ok(session) => {
                                                let token =
                                                    UserToken::generate_token(session).unwrap();
                                                let cookie =
                                                    Cookie::build("session", token).finish();
                                                cookies.add(cookie);

                                                let user = conn
                                                    .run(move |c| {
                                                        User::find_one_by_uuid(user_uuid, c)
                                                    })
                                                    .await;

                                                Outcome::Success(UserGuard(user))
                                            }
                                            Err(_) => {
                                                Outcome::Failure((Status::InternalServerError, ()))
                                            }
                                        }
                                    }
                                    _ => Outcome::Failure((Status::InternalServerError, ())),
                                }
                            }
                            None => Outcome::Failure((Status::InternalServerError, ())),
                        }
                    }
                    Err(error) => {
                        match error.kind() {
                            ErrorKind::ExpiredSignature => {
                                // If the session is expired we shouldn't return the user
                                // however this shouldn't invalidate the request
                                Outcome::Success(UserGuard(None))
                            }
                            _ => Outcome::Failure((Status::Unauthorized, ())),
                        }
                    }
                }
            }
            None => Outcome::Success(UserGuard(None)),
        }
    }
}
