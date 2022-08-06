use crate::db::models::media::Media;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::media::MediaType;
use juniper::{FieldError, Value};
use uuid::Uuid;

pub async fn get_media<'a, 'b>(
    context: &'a GQLContext,
    uuid: Uuid,
    payment_request: Option<String>,
) -> Result<MediaType, FieldError> {
    let connection = context.get_db_connection();
    let result = connection
        .run(move |c| Media::find_one_by_uuid(uuid, c))
        .await;

    match result {
        Ok(result) => match result {
            Some(media) => match payment_request {
                Some(payment_request) => Ok(MediaType::from((media, payment_request))),
                None => Ok(MediaType::from(media)),
            },
            None => Err(FieldError::new(
                "No media found with the provided Uuid",
                Value::null(),
            )),
        },
        Err(_) => Err(FieldError::new("Error while fetching media", Value::null())),
    }
}
