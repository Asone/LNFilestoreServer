#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

extern crate diesel_derive_enum;
extern crate dotenv;
extern crate juniper_rocket_multipart_handler;
extern crate tokio_util;
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

use dotenv::dotenv;
use juniper::EmptySubscription;
use rocket::{
    figment::Figment,
    Rocket,
};

use crate::db::PostgresConn;
use app::Schema;
use app::{
    get_graphql_handler, graphiql, login, options_handler, payable_post_graphql_handler,
    post_graphql_handler, upload
};
use catchers::payment_required::payment_required;
use cors::Cors;
use graphql::{context::GQLContext, mutation::Mutation, query::Query};

itconfig::config! {
    DATABASE_URL: String,
    JWT_TOKEN_SECRET: String

    ROCKET {
        static BASE_URL: String => "/",
    }
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    config::init();

    let figment = Figment::from(rocket::Config::default());
    // .merge(("limits", Limits::new().limit("json", 16.mebibytes())));
    Rocket::build()
        // .configure(figment)
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
                upload,
                login
            ],
        )
        .attach(Cors)
        .manage(Cors)
        .launch()
        .await
        .expect("server to launch");
}
