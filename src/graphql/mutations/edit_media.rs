use juniper::FieldResult;

use crate::{graphql::{context::GQLContext, types::input::media::EditMediaInput}, db::models::media::Media};


pub async fn edit_media<'a>(
        context: &'a GQLContext,
        uuid: uuid::Uuid,
        media: EditMediaInput,
    ) -> FieldResult<bool>  {
        let connection = context.get_db_connection();

        let media = connection.run(move |c| Media::find_one_by_uuid(uuid, c)).await;

        Ok(true)

}