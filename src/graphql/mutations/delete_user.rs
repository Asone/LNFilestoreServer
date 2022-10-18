use juniper::{FieldResult, FieldError};

use crate::{graphql::context::GQLContext, db::models::user::User};

pub async fn delete_user<'a>(
        context: &'a GQLContext,
        uuid: uuid::Uuid
    ) -> FieldResult<bool>  {
        let connection = context.get_db_connection();

        let result = connection.run(move |c| User::delete_user(uuid, c)).await;

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
            },
            Err(_) => Err(FieldError::new(
                "An error happened while trying to delete user",
                juniper::Value::Null,
            ))
        } 

}