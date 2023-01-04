use crate::db::models::media::Media;
use crate::db::models::user::UserRoleEnum;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::media::MediaType;
use juniper::FieldError;

pub async fn get_files_list<'a>(context: &'a GQLContext) -> Result<Vec<MediaType>, FieldError> {
    let connection = context.get_db_connection();
    let db_results = match context.is_authenticated() && context.has_permissioned_role(vec![UserRoleEnum::Admin]) {
        true => {
            connection.run(move |c| Media::find_all(c)).await

        },
        false => {
            connection.run(move |c| Media::find_all_published(c)).await
        }
    };
    Ok(db_results
        .into_iter()
        .map(|p| MediaType::from(p))
        .collect::<Vec<MediaType>>())
}
