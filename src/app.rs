///
/// Todo : Split routes in different files
///
use crate::{
    graphql::{context::GQLContext, mutation::Mutation, query::Query},
    guards::userguard::UserGuard,
    lnd::client::LndClient,
};
use juniper_rocket_multipart_handler::graphql_upload_wrapper::GraphQLUploadWrapper;
use rocket::State;
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GQLContext>>;
use crate::db::PostgresConn;
use crate::guards::paymentrequestheader::PaymentRequestHeader;
use juniper::{EmptySubscription, RootNode};
use juniper_rocket::GraphQLResponse;

/*
    This is a void handler that will return a 200 empty response
    for browsers that intends to check pre-flight for CORS rules.
*/
#[rocket::options("/graphql")]
pub async fn graphql_options_handler() {}

#[rocket::options("/auth")]
pub async fn auth_options_handler() {}

/**
   Calls the API with a query specific paywall protected mechanism.
*/
#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    request: GraphQLUploadWrapper,
    schema: &State<Schema>,
    db: PostgresConn,
    user_guard: UserGuard,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .operations
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: request.files,
                user: user_guard.0,
                server_config: None,
            },
        )
        .await
}

/// Calls the API through an API-scoped paywall
#[rocket::post("/payable", data = "<request>")]
pub async fn payable_post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
    _payment_request: PaymentRequestHeader,
    user_guard: UserGuard,
) -> GraphQLResponse {
    request
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: None,
                user: user_guard.0,
                server_config: None,
            },
        )
        .await
}

#[rocket::post("/upload", data = "<request>")]
pub async fn upload<'r>(
    request: GraphQLUploadWrapper,
    schema: &State<Schema>,
    db: PostgresConn,
    user_guard: UserGuard,
    lnd: LndClient,
) -> GraphQLResponse {
    let result = request
        .operations
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd,
                files: request.files,
                user: user_guard.0,
                server_config: None,
            },
        )
        .await;

    result
}
