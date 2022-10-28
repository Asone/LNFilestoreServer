#![recursion_limit = "1024"]
#[cfg(test)]
mod tests;
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
mod responders;
mod routes;

use crate::db::PostgresConn;
use app::Schema;
use db::igniter::run_db_migrations;
use dotenv::dotenv;
use juniper::EmptySubscription;
use rocket::{Rocket, Build};
use rocket::{fairing::AdHoc, Route};
use routes::{auth::login, file::get_file, utils::graphiql, utils::static_index};
use std::env;

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

    let rocket = app_build().launch()
        .await
        .expect("server to launch");
    Ok(())
}

fn app_build() -> Rocket<Build> {
    let rocket = Rocket::build()
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
        .mount("/", routes_builder());

    return rocket;

}

fn routes_builder() -> Vec<Route> {
    let mut base_routes = rocket::routes![
        graphql_options_handler,
        auth_options_handler,
        post_graphql_handler,
        payable_post_graphql_handler,
        upload,
        login,
        get_file
    ];

    let enable_dev_tools = env::var("ENABLE_DEV_TOOLS").unwrap_or("false".to_string());

    let enable_dev_tools = enable_dev_tools.parse::<bool>().unwrap_or(false);

    let additional_routes = match enable_dev_tools {
        true => {
            routes![graphiql]
        }
        false => {
            routes![static_index]
        }
    };

    base_routes.extend(additional_routes);

    return base_routes;
}
