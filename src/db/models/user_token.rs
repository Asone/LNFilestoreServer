#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub token: String, // pub login_session: String,
}

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

use super::{session::UserSession, user::User};

// #[derive(Debug, Insertable)]
// #[table_name = "session"]
// pub struct  NewUserToken {
//     pub uuid: uuid::Uuid,
//     pub token: String,
//     expires_at: DateTime,
// }

impl UserToken {
    pub fn generate_token(
        user_session: UserSession,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let payload = Self::from(user_session);
        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    }
}

impl From<UserSession> for UserToken {
    fn from(user_session: UserSession) -> Self {
        Self {
            iat: Utc::now().timestamp(),
            exp: user_session.expires_at.timestamp(),
            user: user_session.user_uuid.to_string(),
            token: user_session.token,
        }
    }
}
