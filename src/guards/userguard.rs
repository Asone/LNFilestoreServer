use std::env;

extern crate dotenv;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
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

                                                if user.is_err() {
                                                    return Outcome::Failure((
                                                        Status::InternalServerError,
                                                        (),
                                                    ));
                                                };

                                                Outcome::Success(UserGuard(user.unwrap()))
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
                    Err(_error) => Outcome::Success(UserGuard(None)),
                }
            }
            None => Outcome::Success(UserGuard(None)),
        }
    }
}
