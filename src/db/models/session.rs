pub use crate::db::schema::session;
use crate::errors::authentication::AuthenticationError;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel;
use serde::Serialize;
use uuid::Uuid;

use super::user::User;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, PartialEq, Associations, Serialize)]
#[primary_key(uuid)]
#[table_name = "session"]
#[belongs_to(parent = User, foreign_key = "user")]
pub struct UserSession {
    uuid: Uuid,
    pub token: String,
    pub user_uuid: Uuid,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "session"]
pub struct NewUserSession {
    uuid: Uuid,
    pub token: String,
    pub user_uuid: Uuid,
    pub expires_at: NaiveDateTime,
}

impl From<(String, User)> for NewUserSession {
    fn from(data: (String, User)) -> Self {
        let expires_at = Utc::now() + Duration::days(128);
        Self {
            uuid: Uuid::new_v4(),
            token: Uuid::new_v4().to_string(),
            user_uuid: data.1.uuid,
            expires_at: expires_at.naive_utc(),
        }
    }
}

impl UserSession {
    /// Creates a session stored in database from a NewUserSession instance
    pub fn create(
        new_session: NewUserSession,
        connection: &PgConnection,
    ) -> Result<UserSession, AuthenticationError> {
        use crate::db::schema::session::dsl::*;

        let result = diesel::insert_into::<session>(session)
            .values(&new_session)
            .get_result::<UserSession>(connection);

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(AuthenticationError::DbError(
                "An error happened while creating session".to_string(),
            )),
        }
    }
}
