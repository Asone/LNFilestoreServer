pub use crate::db::schema::session;
use crate::db::schema::user;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

use diesel;

/// User object instance
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
    /// Finds a user based on its login
    pub fn find_one_by_username(username: String, connection: &PgConnection) -> Option<User> {
        use crate::db::schema::user::dsl::*;

        user.filter(login.eq(username))
            .first::<User>(connection)
            .optional()
            .unwrap()
    }

    pub fn find_one_by_uuid(user_uuid: uuid::Uuid, connection: &PgConnection) -> Option<User> {
        use crate::db::schema::user::dsl::*;

        user.filter(uuid.eq(user_uuid))
            .first::<User>(connection)
            .optional()
            .unwrap()
    }
}
