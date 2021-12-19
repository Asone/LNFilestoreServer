#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
extern crate tonic;

mod app;
mod cors;
mod db;
mod graphql;
mod lnd;
mod requests;
use dotenv::dotenv;
use juniper::EmptySubscription;
use rocket::Rocket;

use crate::{
    app::Schema,
    cors::Cors,
    graphql::context::GQLContext,
    graphql::{mutation::Mutation, query::Query},
};

use crate::app::{
    get_graphql_handler, graphiql, options_handler, payable_post_graphql_handler,
    post_graphql_handler,
};
use crate::db::PostgresConn;

itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    Rocket::build()
        .attach(PostgresConn::fairing())
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<GQLContext>::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                options_handler,
                graphiql,
                get_graphql_handler,
                post_graphql_handler,
                payable_post_graphql_handler
            ],
        )
        .attach(Cors)
        .manage(Cors)
        .launch()
        .await
        .expect("server to launch");
}
