use std::env;

use super::PostgresConn;
use crate::db::models::user::NewUser;
use crate::db::models::user::UserRoleEnum;
use bcrypt::hash;
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use rocket::{Build, Rocket};

use diesel;
embed_migrations!();

// This methods runs migrations on ignite of the server
// if `DATABASE_RUN_MIGRATIONS_ON_IGNITE` env var is set as true.
pub async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let conn = PostgresConn::get_one(&rocket)
        .await
        .expect("Database connection");

    let flag = env::var("DATABASE_RUN_MIGRATIONS_ON_IGNITE")
        .unwrap_or("false".to_string())
        .parse::<bool>();

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

// populates the database on ignite of the server
// if `DATABASE_SEED_ON_IGNITE` env var is set to true
pub async fn seed_db(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    use crate::db::schema::user::dsl::*;

    let flag = env::var("DATABASE_SEED_ON_IGNITE")
        .unwrap_or("false".to_string())
        .parse::<bool>();

    if flag.is_err() || !flag.unwrap() {
        return Ok(rocket);
    }

    let conn = PostgresConn::get_one(&rocket)
        .await
        .expect("Database connection");

    let admin_user = default_admin();

    if admin_user.is_none() {
        return Ok(rocket);
    }

    let records = vec![admin_user.unwrap()];

    let _ = conn
        .run(move |c| {
            diesel::insert_into::<user>(user)
                .values(&records)
                .on_conflict(email)
                .do_nothing()
                .execute(&*c)
        })
        .await;

    Ok(rocket)
}

fn default_admin() -> Option<NewUser> {
    let admin_name = env::var("DEFAULT_ADMIN_NAME");
    let admin_email = env::var("DEFAULT_ADMIN_EMAIL");
    let admin_password = env::var("DEFAULT_ADMIN_PWD");

    if admin_name.is_err() || admin_email.is_err() || admin_password.is_err() {
        return None;
    }

    Some(NewUser {
        uuid: uuid::Uuid::new_v4(),
        login: admin_name.unwrap(),
        email: admin_email.unwrap(),
        password: hash(&admin_password.unwrap(), DEFAULT_COST).unwrap(),
        role: UserRoleEnum::Admin,
    })
}
