use std::env;

use super::PostgresConn;
use rocket::{Build, Rocket};
embed_migrations!();
/// This method should be called on ignite of the server - juste before server launch -.
/// It calls the migrations scripts to populate the database with tables and default data.
/// Note that it is up to the SQL scripts in migrations to ensure that it does not override
/// previous existing tables or data when executed.
pub async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let conn = PostgresConn::get_one(&rocket)
        .await
        .expect("Database connection");

    let flag = env::var("DATABASE_RUN_MIGRATIONS_ON_IGNITE")
        .unwrap_or("false".to_string())
        .parse::<bool>();

    // Skip
    if flag.is_err() || !flag.unwrap() {
        return Ok(rocket);
    }

    conn.run(|conn| match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}
