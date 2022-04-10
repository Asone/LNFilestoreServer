use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::session::UserSession;

/// Represents a user token
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

impl UserToken {
    /// Encodes the UserToken object to a JWT Token
    pub fn generate_token(
        user_session: UserSession,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let payload = Self::from(user_session);
        // let header = Header::new(Algorithm::HS256);

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
