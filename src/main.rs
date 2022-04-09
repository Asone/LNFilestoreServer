#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

extern crate diesel_derive_enum;
extern crate dotenv;
extern crate tonic;

mod app;
mod catchers;
mod cors;
mod db;
mod errors;
mod forms;
mod graphql;
mod guards;
mod lnd;
mod requests;

use catchers::payment_required::payment_required;
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
    admin, get_graphql_handler, graphiql, login, options_handler, payable_post_graphql_handler,
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
        .register("/", catchers![payment_required])
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
                payable_post_graphql_handler,
                login,
                admin
            ],
        )
        .attach(Cors)
        .manage(Cors)
        .launch()
        .await
        .expect("server to launch");
}
