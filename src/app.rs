///
/// Todo : Split routes in different files
///
use crate::{
    db::models::{session::UserSession, user_token::UserToken},
    errors::authentication::AuthenticationError,
    forms::login_user::LoginUser,
    graphql::{context::GQLContext, mutation::Mutation, query::Query},
    guards::user_guard::UserGuard,
    lnd::client::LndClient,
};

use rocket::{
    figment::value::Value,
    form::{Form, Strict},
    http::{Cookie, Status},
    response::{
        content::{self, Json},
        status, Result,
    },
    Response, State,
};
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GQLContext>>;
use crate::db::PostgresConn;
use crate::requests::header::PaymentRequestHeader;
use juniper::{EmptySubscription, RootNode};
use juniper_rocket::GraphQLResponse;
use rocket::http::CookieJar;

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
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
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
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}

/// Authentication route
#[rocket::post("/auth", data = "<user_form>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    user_form: Form<Strict<LoginUser>>,
    db: PostgresConn,
) -> rocket::http::Status {
    let user = user_form.into_inner().into_inner();

    let session = user.login(db).await;

    match session {
        Ok(user_session) => {
            let token = UserToken::generate_token(user_session).unwrap();
            let cookie = Cookie::build("session", token).finish();

            cookies.add(cookie);
            Status::Ok
        }
        Err(_) => Status::ExpectationFailed,
    }
}

/// User Authentication protected route
/// It shall provide an access to full graphql schema
///
#[rocket::post("/admin", data = "<request>")]
pub async fn admin(
    user_guard: UserGuard,
    cookies: &CookieJar<'_>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
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
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}
