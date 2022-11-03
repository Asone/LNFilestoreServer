use juniper::{FieldError, FieldResult, Value};

use crate::db::models::user::User;
use crate::db::models::user::UserRoleEnum;
use crate::graphql::types::input::user::EditUserInput;

use super::{
    context::GQLContext, types::input::file::FileInput, types::input::media::EditMediaInput,
    types::input::user::NewUserInput, types::output::media::MediaType,
    types::output::user::UserType,
};
use crate::graphql::mutations::create_user;
use crate::graphql::mutations::delete_media;
use crate::graphql::mutations::delete_user;
use crate::graphql::mutations::edit_media;
use crate::graphql::mutations::edit_user;
use crate::graphql::mutations::update_password;
use crate::graphql::mutations::upload_file;

pub struct Mutation;

impl Mutation {
    pub fn is_authenticated(user: &Option<User>) -> bool {
        match user {
            Some(_) => true,
            None => false,
        }
    }

    pub fn has_permissioned_role(user: &Option<User>, roles: Vec<UserRoleEnum>) -> bool {
        match user {
            Some(user) => roles.contains(&user.role),
            None => false,
        }
    }
}

#[juniper::graphql_object(context = GQLContext)]
impl Mutation {
    #[graphql(description = "Creates a user")]
    async fn create_user<'a>(
        context: &'a GQLContext,
        new_user_input: NewUserInput,
    ) -> FieldResult<UserType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }

        create_user::create_user(context, new_user_input).await
    }

    #[protected]
    #[graphql(description = "Edits a user")]
    async fn edit_user<'a>(
        context: &'a GQLContext,
        uuid: uuid::Uuid,
        edit_user_input: EditUserInput,
    ) -> FieldResult<UserType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }

        if !Self::has_permissioned_role(&context.user, vec![UserRoleEnum::Admin]) {
            return Err(FieldError::new(
                "You do not have the required permission to perform this action",
                Value::null(),
            ));
        }

        edit_user::edit_user(context, uuid, edit_user_input).await
    }

    #[graphql(description = "Deletes a user")]
    async fn delete_user<'a>(context: &'a GQLContext, uuid: uuid::Uuid) -> FieldResult<bool> {
        if !Self::is_authenticated(&context.user) {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }
        if !Self::has_permissioned_role(&context.user, vec![UserRoleEnum::Admin]) {
            return Err(FieldError::new(
                "You do not have the required permission to perform this action",
                Value::null(),
            ));
        }

        delete_user::delete_user(context, uuid).await
    }

    #[graphql(description = "Upload and stores a payable media onto the server")]
    async fn upload_file<'a>(
        context: &'a GQLContext,
        file_input: FileInput,
    ) -> FieldResult<MediaType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }

        upload_file::upload_file(context, file_input).await
    }

    #[graphql(description = "Edit a media")]
    async fn edit_media<'a>(
        context: &'a GQLContext,
        uuid: uuid::Uuid,
        media: EditMediaInput,
    ) -> FieldResult<MediaType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }

        if !Self::has_permissioned_role(
            &context.user,
            vec![UserRoleEnum::Admin, UserRoleEnum::Moderator],
        ) {
            return Err(FieldError::new(
                "You do not have the required permission to perform this action",
                Value::null(),
            ));
        }

        edit_media::edit_media(context, uuid, media).await
    }

    #[graphql(description = "Deletes a media")]
    async fn delete_media<'a>(context: &'a GQLContext, uuid: uuid::Uuid) -> FieldResult<bool> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                Value::null(),
            ));
        }

        if !Self::has_permissioned_role(&context.user, vec![UserRoleEnum::Admin]) {
            return Err(FieldError::new(
                "You do not have the required permission to perform this action",
                Value::null(),
            ));
        }

        delete_media::delete_media(context, uuid).await
    }

    // Changes password for current user
    async fn change_password<'a>(context: &'a GQLContext, password: String) -> FieldResult<bool> {
        update_password::update_password(context, password).await
    }
}
