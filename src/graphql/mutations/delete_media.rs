use juniper::{FieldError, FieldResult};

use crate::{db::models::media::Media, graphql::context::GQLContext};

pub async fn delete_media<'a>(context: &'a GQLContext, uuid: uuid::Uuid) -> FieldResult<bool> {
    let connection = context.get_db_connection();

    let result = connection.run(move |c| Media::delete(uuid, c)).await;

    match result {
        Ok(count) => {
            if count == 1 {
                Ok(true)
            } else {
                Err(FieldError::new(
                    "An error happened while trying to delete media",
                    juniper::Value::Null,
                ))
            }
        }
        Err(_) => Err(FieldError::new(
            "An error happened while trying to delete media",
            juniper::Value::Null,
        )),
    }
}
