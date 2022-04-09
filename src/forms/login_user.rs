use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        models::{
            session::{NewUserSession, UserSession},
            user::User,
        },
        PostgresConn,
    },
    errors::authentication::AuthenticationError,
};

#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    // #[serde(skip_serializing)]
    pub password: String,
}

impl LoginUser {
    pub async fn login(self, db: PostgresConn) -> Result<UserSession, AuthenticationError> {
        let password = self.password.clone();
        let user = db.run(|c| User::find_one_by_login(self.username, c)).await;

        match user {
            Some(user) => match password.as_str() == user.password.as_str() {
                true => {
                    db.run(move |c| {
                        UserSession::create(NewUserSession::from(("toto".to_string(), user)), c)
                    })
                    .await
                }
                false => Err(AuthenticationError::PasswordMismatch(
                    "Passwords dont match".to_string(),
                )),
            },
            None => Err(AuthenticationError::UserNotFound(
                "User not found".to_string(),
            )),
        }
    }
}
