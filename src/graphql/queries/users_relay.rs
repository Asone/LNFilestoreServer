use crate::db::models::user::User;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::user::UserType;
use juniper::{FieldError, FieldResult, Value};
use juniper_relay_connection::RelayConnection;

pub async fn users_relay<'a, 'b>(
    context: &'a GQLContext,
    first: Option<i32>,
    after: Option<String>,
    last: Option<i32>,
    before: Option<String>,
) -> FieldResult<RelayConnection<UserType>> {
    let connection = context.get_db_connection();
    let db_results = connection.run(move |c| User::find(c)).await;

    match db_results {
        Ok(results) => RelayConnection::new(first, after, last, before, |_, _, _| {
            Ok(results
                .into_iter()
                .map(|p| UserType::from(p))
                .collect::<Vec<UserType>>())
        }),
        Err(_) => Err(FieldError::new(
            "Error while fetching the users list",
            Value::null(),
        )),
    }
}
