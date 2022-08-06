use crate::graphql::types::output::post::PreviewPostType;
use crate::{db::models::Post, graphql::context::GQLContext};
use juniper::FieldError;

pub async fn get_posts_list<'a>(
    context: &'a GQLContext,
) -> Result<Vec<PreviewPostType>, FieldError> {
    let connection = context.get_db_connection();
    let db_results = connection.run(move |c| Post::find_all_published(c)).await;

    Ok(db_results
        .into_iter()
        .map(|p| PreviewPostType::from(p))
        .collect::<Vec<PreviewPostType>>())
}
