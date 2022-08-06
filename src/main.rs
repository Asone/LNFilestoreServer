#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

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
mod routes;

use crate::db::PostgresConn;
use app::Schema;
use db::igniter::run_db_migrations;
use dotenv::dotenv;
use juniper::EmptySubscription;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use routes::{auth::login, file::get_file, utils::graphiql};

use app::{
    auth_options_handler, graphql_options_handler, payable_post_graphql_handler,
    post_graphql_handler, upload,
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
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    config::init();

    let _rocket = Rocket::build()
        .attach(PostgresConn::fairing())
        .attach(Cors)
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .manage(Cors)
        // .configure(figment)
        .register("/", catchers![payment_required])
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<GQLContext>::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                graphql_options_handler,
                auth_options_handler,
                graphiql,
                post_graphql_handler,
                payable_post_graphql_handler,
                upload,
                login,
                get_file
            ],
        )
        .launch()
        .await
        .expect("server to launch");

    Ok(())
}
