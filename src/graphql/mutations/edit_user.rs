use juniper::{FieldError, FieldResult, Value};

use crate::{
    db::models::user::{EditUser, User},
    graphql::{
        context::GQLContext,
        types::{input::user::EditUserInput, output::user::UserType},
    },
};

pub async fn edit_user<'a>(
    context: &'a GQLContext,
    uuid: uuid::Uuid,
    edited_user_input: EditUserInput,
) -> FieldResult<UserType> {
    let connection = context.get_db_connection();

    let user = connection
        .run(move |c| User::find_one_by_uuid(uuid, c))
        .await;

    if user.is_err() {
        return Err(FieldError::new(
            "An error happened while fetch the media target in db",
            Value::null(),
        ));
    };

    let user = user.unwrap();

    match user {
        Some(user) => {
            let edit_user = EditUser::from(edited_user_input);
            let result = connection
                .run(move |c| User::update(user.uuid, edit_user, c))
                .await;

            if result.is_err() {
                return Err(FieldError::new("Error while updating user", Value::null()));
            }

            let result = result.unwrap();

            Ok(UserType::from(result))
        }
        None => {
            return Err(FieldError::new(
                "No user found with the provided uuid",
                Value::null(),
            ));
        }
    }
}
