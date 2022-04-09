use diesel::PgConnection;
use rocket_sync_db_pools::{database, diesel};

use self::models::api_payment::ApiPayment;

pub mod models;
pub mod schema;

#[database("main_db")]
pub struct PostgresConn(pub diesel::PgConnection);

impl PostgresConn {
    pub async fn find_api_payment(self, payment_request: String) -> Option<ApiPayment> {
        self.run(|c| ApiPayment::find_one_by_request(payment_request, c))
            .await
    }
}
