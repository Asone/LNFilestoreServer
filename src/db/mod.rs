use rocket_sync_db_pools::{database, diesel};

pub mod models;
pub mod schema;

#[database("main_db")]
pub struct PostgresConn(pub diesel::PgConnection);
