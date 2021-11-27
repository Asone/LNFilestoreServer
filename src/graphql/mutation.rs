use juniper::{FieldError, FieldResult};

use crate::db::models::post::{NewPost, Post};

use super::{
    context::GQLContext,
    types::{input::post::CreatePostInput, output::post::PostType},
};

pub struct Mutation;

#[juniper::graphql_object(context = GQLContext)]
impl Mutation {
    async fn create_post<'a>(
        context: &'a GQLContext,
        post: CreatePostInput,
    ) -> FieldResult<PostType> {
        // context.pool
        let connection = context.get_db_connection();
        let result = connection
            .run(|c| Post::create(NewPost::from(post), c))
            .await;

        match result {
            Ok(r) => Ok(PostType::from(r)),
            Err(e) => {
                let error = r#"{}"#;
                Err(FieldError::new(e, graphql_value!(error)))
            }
        }
    }
}
