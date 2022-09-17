use crate::db::models::user::User;
use crate::graphql::context::GQLContext;
use juniper::{FieldError, Value};

pub async fn update_password<'a>(
    context: &'a GQLContext,
    new_password: String,
) -> Result<bool, FieldError> {
    let user = context.user.clone();

    let connection = context.get_db_connection();

    match user {
        Some(user) => {
            let result = connection
                .run(move |c| User::change_password(user.uuid, new_password, c))
                .await;

            match result {
                Ok(_user) => Ok(true),
                Err(_) => Err(FieldError::new(
                    "An error happened on updating password",
                    Value::null(),
                )),
            }
        }
        None => Err(FieldError::new(
            "You need to be authenticated first.",
            Value::null(),
        )),
    }
}
