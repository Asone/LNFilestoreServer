use juniper::{FieldError, FieldResult, Value};

use crate::{
    db::models::media::Media,
    graphql::{
        context::GQLContext,
        types::{input::media::EditMediaInput, output::media::MediaType},
    },
};

pub async fn edit_media<'a>(
    context: &'a GQLContext,
    uuid: uuid::Uuid,
    edited_media_input: EditMediaInput,
) -> FieldResult<MediaType> {
    let connection = context.get_db_connection();

    let media = connection
        .run(move |c| Media::find_one_by_uuid(uuid, c))
        .await;

    if media.is_err() {
        return Err(FieldError::new(
            "An error happened while fetch the media target in db",
            Value::null(),
        ));
    };

    let media = media.unwrap();

    match media {
        Some(media) => {
            let result = connection
                .run(move |c| Media::update(media.uuid, edited_media_input, c))
                .await;
            match result {
                Ok(result) => Ok(MediaType::from(result)),
                Err(_) => {
                    return Err(FieldError::new(
                        "An error happened while updating media in  db",
                        Value::null(),
                    ));
                }
            }
        }
        None => {
            return Err(FieldError::new(
                "No media found with provided uuid",
                Value::null(),
            ));
        }
    }
}
