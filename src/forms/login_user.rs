use bcrypt::{hash, verify, DEFAULT_COST};
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

/// Login form
#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl LoginUser {
    pub async fn login(self, db: PostgresConn) -> Result<(User, UserSession), AuthenticationError> {
        let password = self.password.clone();
        let user = db
            .run(|c| User::find_one_by_username(self.username, c))
            .await;

        let hashed_password = hash(&password, DEFAULT_COST).unwrap();
        match user {
            Some(user) => match verify(password.as_str(), hashed_password.as_str()) {
                Ok(result) => match result {
                    true => {
                        let user_object = user.clone();
                        let user_session = db
                            .run(move |c| UserSession::create(NewUserSession::from(&user), c))
                            .await;
                        match user_session {
                            Ok(session) => Ok((user_object, session)),
                            Err(e) => Err(e),
                        }
                    }
                    false => Err(AuthenticationError::PasswordMismatch(
                        "Passwords dont match".to_string(),
                    )),
                },
                Err(_) => Err(AuthenticationError::InternalDecryptionError),
            },
            None => Err(AuthenticationError::UserNotFound(
                "User not found".to_string(),
            )),
        }
    }

    pub async fn get_user(self, db: PostgresConn) -> Option<User> {
        db.run(|c| User::find_one_by_username(self.username, c))
            .await
    }
}
