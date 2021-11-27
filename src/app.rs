use crate::{
    graphql::{context::GQLContext, mutation::Mutation, query::Query},
    lnd::client::LndClient,
};
use rocket::{response::content, State};
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GQLContext>>;

use crate::db::PostgresConn;
use juniper::{EmptySubscription, RootNode};
use juniper_rocket::GraphQLResponse;

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

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}
