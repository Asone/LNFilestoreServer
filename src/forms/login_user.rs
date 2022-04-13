use serde::{Deserialize, Serialize};
use bcrypt::{DEFAULT_COST, hash, verify};

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

/// Login form
#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    // #[serde(skip_serializing)]
    pub password: String,
}

impl LoginUser {
    pub async fn login(self, db: PostgresConn) -> Result<UserSession, AuthenticationError> {
        let password = self.password.clone();
        let user = db
            .run(|c| User::find_one_by_username(self.username, c))
            .await;

        let hashed_password = hash(&password, DEFAULT_COST).unwrap();
        match user {
            Some(user) => match verify(password.as_str(), user.password.as_str()) {
                Ok(result) => match result {

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
                Err(_) => Err(AuthenticationError::InternalDecryptionError)
                
            },
            None => Err(AuthenticationError::UserNotFound(
                "User not found".to_string(),
            )),
        }
    }
}
