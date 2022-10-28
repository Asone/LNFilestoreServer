pub fn rocket_instance() -> Rocket<Build> {
 Rocket::build()
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
        .mount("/", routes_builder())
        .launch()
        .await
        .expect("server to launch")
}