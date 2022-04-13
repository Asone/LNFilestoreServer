use juniper::{FieldError, FieldResult};

use crate::db::models::{
    post::{NewPost, Post},
    user::User,
};

use super::{
    context::GQLContext,
    types::{input::post::CreatePostInput, output::post::PostType},
};

pub struct Mutation;

impl Mutation {
    pub fn is_authenticated(user: &Option<User>) -> bool {
        match user {
            Some(_) => true,
            None => false,
        }
    }
}

#[juniper::graphql_object(context = GQLContext)]
impl Mutation {
    #[graphql(
        description = "Creates a post. This mutation is available only for authenticated users."
    )]
    async fn create_post<'a>(
        context: &'a GQLContext,
        post: CreatePostInput,
    ) -> FieldResult<PostType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                graphql_value!(""),
            ));
        }

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


    /// Changes password for current user
    async fn change_password<'a>(
        context: &'a GQLContext,
        password: String
    ) ->  FieldResult<bool> {

        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to activate this mutation",
                graphql_value!(""),
            ));
        }

        let user = context.get_user().to_owned().unwrap();
        let connection = context.get_db_connection();
        let result = connection.run(move |c| User::change_password(user.uuid, password, c)).await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
        
    }
}
