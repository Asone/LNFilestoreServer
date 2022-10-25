use chrono::NaiveDateTime;

use crate::db::models::user::User;

pub struct UserType {
    pub uuid: uuid::Uuid,
    pub login: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserType {
    fn from(user: User) -> Self {
        Self {
            uuid: user.uuid,
            login: user.login,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
