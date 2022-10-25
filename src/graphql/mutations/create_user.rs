use juniper::{FieldError, FieldResult, Value};

use crate::{
    db::models::user::{NewUser, User},
    graphql::{
        context::GQLContext,
        types::{input::user::NewUserInput, output::user::UserType},
    },
};

pub async fn create_user<'a>(
    context: &'a GQLContext,
    new_user_input: NewUserInput,
) -> FieldResult<UserType> {
    let connection = context.get_db_connection();

    let login = new_user_input.login.clone();
    let email = new_user_input.email.clone();

    let result = connection
        .run(move |c| User::find_one_by_username_or_email(login, email, c))
        .await;

    if result.is_err() {
        return Err(FieldError::new(
            "An error happened while ensure user does not exist in db",
            Value::null(),
        ));
    };

    let result = result.unwrap();

    match result {
        Some(_) => {
            return Err(FieldError::new(
                "A user already exists with the provided user or email",
                Value::null(),
            ));
        }
        None => {
            let connection = context.get_db_connection();

            let user = connection
                .run(move |c| User::create(NewUser::from(new_user_input), c))
                .await;

            if user.is_err() {
                return Err(FieldError::new(
                    "An error happened while registering new user",
                    Value::null(),
                ));
            }

            Ok(UserType::from(user.unwrap()))
        }
    }
}
