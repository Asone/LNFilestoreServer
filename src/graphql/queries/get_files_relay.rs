use juniper::FieldResult;
use juniper_relay_connection::RelayConnection;

use crate::{
    db::models::{media::Media, user::UserRoleEnum},
    graphql::{context::GQLContext, types::output::media::MediaType},
};

pub async fn get_files_list_relay<'a>(
    context: &'a GQLContext,
    first: Option<i32>,
    after: Option<String>,
    last: Option<i32>,
    before: Option<String>,
) -> FieldResult<RelayConnection<MediaType>> {
    let connection = context.get_db_connection();

    let db_results = match context.is_authenticated() && context.has_permissioned_role(vec![UserRoleEnum::Admin]) {
        true => {
            connection.run(move |c| Media::find_all(c)).await

        },
        false => {
            connection.run(move |c| Media::find_all_published(c)).await
        }
    };

    // Ok(RelayConnection::empty())
    let result = RelayConnection::new(first, after, last, before, |first, after, last| {
        Ok(db_results
            .into_iter()
            .map(|p| MediaType::from(p))
            .collect::<Vec<MediaType>>())
    });

    result
}
