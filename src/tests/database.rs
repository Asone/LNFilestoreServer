#[cfg(test)]
pub mod tests {


    use diesel::{PgConnection, Connection, sql_query, RunQueryDsl};
    use diesel_migrations::RunMigrationsError;

    pub struct TestDb {
        default_db_url: String,
        url: String,
        name: String,
        delete_on_drop: bool,
    }
    impl TestDb {
        pub fn new() -> Self {

            let db_name = format!(
                "test_{}",
                env!("CARGO_PKG_NAME")
            );
            let default_db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL");
            let conn = PgConnection::establish(&default_db_url).unwrap();
            
            sql_query(format!("DROP DATABASE IF EXISTS {};", &db_name))
                .execute(&conn)
                .unwrap();

            sql_query(format!("CREATE DATABASE {};", db_name))
                .execute(&conn)
                .unwrap();

            let url = format!("{}/{}",&default_db_url,&db_name);

            let _migrations = Self::run_migrations(&url);

            Self {
                default_db_url,
                url,
                name: db_name,
                delete_on_drop: true,
            }
        }

        
        pub fn url(&self) -> &str {
            &self.url
        }

        pub fn run_migrations(db_url: &String) -> Result<(), RunMigrationsError> {
            let conn = PgConnection::establish(db_url).unwrap();
            
            diesel_migrations::run_pending_migrations(&conn)
        }

        // For further implementation of fixtures loading in test DB.
        // pub fn load_fixtures(&self) -> () {
        //     let connection = &self.conn();

            
        // }

        pub fn conn(&self) -> PgConnection {
            PgConnection::establish(&self.url.as_str()).unwrap()
        }

        pub fn leak(&mut self) {
            self.delete_on_drop = false;
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if !self.delete_on_drop {
                warn!("TestDb leaking database {}", self.name);
                return;
            }
            let conn = 
                PgConnection::establish(&self.default_db_url).unwrap();
            sql_query(format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                self.name
            ))
            .execute(&conn)
            .unwrap();
            sql_query(format!("DROP DATABASE {}", self.name))
                .execute(&conn)
                .unwrap();
        }
    }

    
    #[test]
    fn test_create() {
        
    }
}
