use juniper::{FieldError, FieldResult};

use crate::{db::models::user::User, graphql::context::GQLContext};

pub async fn delete_user<'a>(context: &'a GQLContext, uuid: uuid::Uuid) -> FieldResult<bool> {
    let connection = context.get_db_connection();

    let result = connection.run(move |c| User::delete(uuid, c)).await;

    match result {
        Ok(count) => {
            if count == 1 {
                Ok(true)
            } else {
                Err(FieldError::new(
                    "An error happened while trying to delete user",
                    juniper::Value::Null,
                ))
            }
        }
        Err(_) => Err(FieldError::new(
            "An error happened while trying to delete user",
            juniper::Value::Null,
        )),
    }
}
