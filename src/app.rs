use std::collections::HashMap;

use crate::{
    graphql::{context::GQLContext, mutation::Mutation, query::Query},
    lnd::client::LndClient,
};
use rocket::{
    data::ToByteUnit, form::Form, fs::TempFile, http::ContentType, response::content, Data, State,
};
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GQLContext>>;

use crate::db::PostgresConn;
use crate::requests::header::PaymentRequestHeader;
use juniper::{EmptySubscription, RootNode};
use juniper_rocket::GraphQLResponse;
use serde_json::Value;

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

/*
    This is a void handler that will return a 200 empty response
    for browsers that intends to check pre-flight for CORS rules.
*/
#[rocket::options("/graphql")]
pub async fn options_handler() {}

/**
   Calls the GraphQL API from a HTTP GET Request.
   It does nothing special but a paywall mechanism through
   a payment_request param could be implemented later.
*/
#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: None,
            },
        )
        .await
}

/**
   Calls the API with a query specific paywall protected mechanism.
*/
#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: None,
            },
        )
        .await
}

/**
   Calls the API through an API-scoped paywall
*/
#[rocket::post("/payable", data = "<request>")]
pub async fn payable_post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
    _payment_request: PaymentRequestHeader,
) -> GraphQLResponse {
    request
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: None,
            },
        )
        .await
}

#[rocket::post("/upload", data = "<request>")]
pub async fn upload<'r>(
    request: crate::graphql::multipart::upload_request::GraphQLUploadRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    let files = request.files.clone();

    request
        .execute(
            &*schema,
            &GQLContext {
                pool: db,
                lnd: lnd,
                files: files,
            },
        )
        .await
}
