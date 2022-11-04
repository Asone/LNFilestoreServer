use chrono::NaiveDateTime;
use juniper_relay_connection::RelayConnectionNode;

use crate::db::models::user::{User, UserRoleEnum};
use crate::graphql::context::GQLContext;

#[derive(juniper::GraphQLEnum)]
pub enum UserRoleEnumType {
    Admin,
    Moderator,
    Publisher,
}

impl From<UserRoleEnum> for UserRoleEnumType {
    fn from(role: UserRoleEnum) -> Self {
        match role {
            UserRoleEnum::Admin => Self::Admin,
            UserRoleEnum::Moderator => Self::Moderator,
            UserRoleEnum::Publisher => Self::Publisher,
        }
    }
}

pub struct UserType {
    pub uuid: uuid::Uuid,
    pub login: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: UserRoleEnumType,
}

impl From<User> for UserType {
    fn from(user: User) -> Self {
        Self {
            uuid: user.uuid,
            login: user.login,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: UserRoleEnumType::from(user.role),
        }
    }
}

#[graphql_object(
    name = "User",
    description = "User objects of database",
    context = GQLContext
)]
impl UserType {
    #[graphql(description = "The user internal id")]
    fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    #[graphql(description = "The user login")]
    fn login(&self) -> &String {
        &self.login
    }

    #[graphql(description = "The user email")]
    fn email(&self) -> &String {
        &self.email
    }

    #[graphql(description = "Creation date of user")]
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    #[graphql(description = "Creation date of media")]
    fn role(&self) -> &UserRoleEnumType {
        &self.role
    }
}

/// Implements relay connection for User
/// It allows using obscure cursors for pagination
impl RelayConnectionNode for UserType {
    type Cursor = String;

    fn cursor(&self) -> Self::Cursor {
        let cursor = format!("user:{}", self.uuid);
        base64::encode(cursor)
    }

    fn connection_type_name() -> &'static str {
        "UserConnection"
    }

    fn edge_type_name() -> &'static str {
        "UserConnectionEdge"
    }
}
