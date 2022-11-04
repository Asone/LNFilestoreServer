use std::env;

pub use crate::db::schema::session;
use crate::errors::authentication::AuthenticationError;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
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
    pub uuid: Uuid,
    pub token: String,
    pub user_uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "session"]
pub struct NewUserSession {
    pub uuid: Uuid,
    pub token: String,
    pub user_uuid: Uuid,
    pub expires_at: NaiveDateTime,
}

impl NewUserSession {}

impl From<User> for NewUserSession {
    fn from(user: User) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            token: Uuid::new_v4().to_string(),
            user_uuid: user.uuid,
            expires_at: UserSession::expiry_generator(None).naive_utc(),
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

    pub fn update_session_expiry(
        session_uuid: uuid::Uuid,
        connection: &PgConnection,
    ) -> QueryResult<UserSession> {
        use crate::db::schema::session::dsl::*;

        let result = diesel::update(session.filter(uuid.eq(session_uuid)))
            .set(expires_at.eq(Self::expiry_generator(None)))
            .get_result::<UserSession>(connection);

        result
        // match result {
        //     Ok(result) => Ok(result),
        //     Err(_) => Err(()),
        // }
    }

    fn expiry_generator(minutes_duration: Option<i64>) -> DateTime<Utc> {
        let duration = match minutes_duration {
            Some(duration) => duration,
            None => match env::var("JWT_TOKEN_DURATION") {
                Ok(duration) => duration.parse::<i64>().unwrap(),
                Err(_) => 1000 as i64,
            },
        };

        Utc::now() + Duration::minutes(duration)
    }
}
