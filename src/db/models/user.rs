use std::error::Error;

pub use crate::db::schema::user;
use crate::db::PostgresConn;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub use crate::db::schema::session;
use crate::errors::authentication::AuthenticationError;

use diesel;

use super::user_token::UserToken;

#[derive(Identifiable, Queryable, PartialEq, Associations)]
#[primary_key(uuid)]
#[table_name = "session"]
#[belongs_to(parent = User, foreign_key = "user")]
pub struct UserSessionRow {
    uuid: Uuid,
    token: String,
    user_uuid: Uuid,
    expires_at: NaiveTime,
}

#[derive(Identifiable, Queryable, PartialEq, Associations, Serialize)]
#[primary_key(uuid)]
#[table_name = "session"]
#[belongs_to(parent = User, foreign_key = "user")]
pub struct UserSession {
    uuid: Uuid,
    token: String,
    user_uuid: Uuid,
    expires_at: NaiveDateTime,
    created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "session"]
pub struct NewUserSession {
    uuid: Uuid,
    token: String,
    user_uuid: Uuid,
    expires_at: NaiveDateTime,
}

impl From<(String, User)> for NewUserSession {
    fn from(data: (String, User)) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            token: data.0,
            expires_at: Utc::now().naive_utc(),
            user_uuid: data.1.uuid,
        }
    }
}

#[derive(Identifiable, Queryable, PartialEq)]
#[primary_key(uuid)]
#[table_name = "user"]
pub struct User {
    pub uuid: uuid::Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn find_one_by_login(login: String, connection: &PgConnection) -> Option<User> {
        use crate::db::schema::user::dsl::*;

        user.filter(login.eq(login))
            .first::<User>(connection)
            .optional()
            .unwrap()
    }
}

#[derive(FromForm, Serialize, Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub login: String,
    // #[serde(skip_serializing)]
    pub password: String,
}

impl LoginUser {
    pub fn generate_token() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn persist_session(
        new_session: NewUserSession,
        connection: &PgConnection,
    ) -> QueryResult<UserSession> {
        use crate::db::schema::session::dsl::*;

        diesel::insert_into::<session>(session)
            .values(&new_session)
            .get_result::<UserSession>(connection)
    }

    pub fn generate_login_session(user: User, conn: &PgConnection) -> QueryResult<UserSession> {
        let token = Self::generate_token();
        let new_session = NewUserSession::from((token, user));

        Self::persist_session(new_session, conn)
    }

    pub fn find_one_by_uuid(
        session_uuid: Uuid,
        connection: &PgConnection,
    ) -> QueryResult<UserSession> {
        use crate::db::schema::session::dsl::*;

        session.filter(uuid.eq(session_uuid)).first(connection)
        // .first::<UserSessionRow>(connection)
    }
}
