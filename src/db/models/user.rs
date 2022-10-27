pub use crate::db::schema::session;
use crate::db::schema::user;
use crate::graphql::types::input::user::EditUserInput;
use crate::graphql::types::input::user::NewUserInput;
use bcrypt::hash;
use bcrypt::DEFAULT_COST;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct EditUser {
    pub email: Option<String>,
}

impl From<EditUserInput> for EditUser {
    fn from(user: EditUserInput) -> Self {
        Self { email: user.email }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password: String,
}

impl From<NewUserInput> for NewUser {
    fn from(new_user: NewUserInput) -> Self {
        Self {
            login: new_user.login,
            email: new_user.email,
            password: new_user.password,
        }
    }
}

/// User object instance
#[derive(Identifiable, Queryable, PartialEq, Clone)]
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
    pub fn create(new_user: NewUser, connection: &PgConnection) -> QueryResult<User> {
        use crate::db::schema::user::dsl::*;

        diesel::insert_into::<user>(user)
            .values(&new_user)
            .get_result(connection)
    }

    pub fn update(
        user_uuid: uuid::Uuid,
        edited_user: EditUser,
        connection: &PgConnection,
    ) -> QueryResult<User> {
        use crate::db::schema::user::dsl::*;

        diesel::update(user.filter(uuid.eq(user_uuid)))
            .set(edited_user)
            .get_result::<User>(connection)
    }

    pub fn delete(user_uuid: uuid::Uuid, connection: &PgConnection) -> Result<usize, Error> {
        use crate::db::schema::user::dsl::*;

        diesel::delete(user.filter(uuid.eq(user_uuid))).execute(connection)
    }

    pub fn change_password(
        user_uuid: uuid::Uuid,
        new_password: String,
        connection: &PgConnection,
    ) -> QueryResult<User> {
        use crate::db::schema::user::dsl::*;
        let hashed_password = hash(new_password, DEFAULT_COST).unwrap();

        diesel::update(user.filter(uuid.eq(user_uuid)))
            .set(password.eq(hashed_password))
            .get_result::<User>(connection)
    }

    pub fn find_one_by_username(username: String, connection: &PgConnection) -> Option<User> {
        use crate::db::schema::user::dsl::*;

        user.filter(login.eq(username))
            .first::<User>(connection)
            .optional()
            .unwrap()
    }

    pub fn find_one_by_uuid(
        user_uuid: uuid::Uuid,
        connection: &PgConnection,
    ) -> QueryResult<Option<User>> {
        use crate::db::schema::user::dsl::*;

        user.filter(uuid.eq(user_uuid))
            .first::<User>(connection)
            .optional()
    }

    pub fn find_one_by_username_or_email(
        username: String,
        user_email: String,
        connection: &PgConnection,
    ) -> QueryResult<Option<User>> {
        use crate::db::schema::user::dsl::*;

        user.filter(login.eq(username))
            .or_filter(email.eq(user_email))
            .first::<User>(connection)
            .optional()
    }
}
