pub use crate::db::schema::api_payment;
use crate::db::PostgresConn;
use crate::lnd::client::LndClient;
use crate::lnd::invoice::{InvoiceParams, InvoiceUtils, LndInvoice};
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

#[derive(Queryable, PartialEq, Associations)]
#[table_name = "api_payment"]
pub struct ApiPayment {
    pub uuid: uuid::Uuid,
    pub request: String,
    pub state: Option<String>,
    pub hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "api_payment"]
pub struct NewApiPayment {
    uuid: Uuid,
    hash: String,
    request: String,
    expires_at: NaiveDateTime,
    state: Option<String>,
}

impl From<LndInvoice> for NewApiPayment {
    fn from(data: LndInvoice) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            request: data.payment_request,
            hash: data.r_hash,
            expires_at: data.expires_at,
            state: None,
        }
    }
}

impl ApiPayment {
    pub fn create(
        new_api_payment: NewApiPayment,
        connection: &PgConnection,
    ) -> QueryResult<ApiPayment> {
        use crate::db::schema::api_payment::dsl::*;

        diesel::insert_into::<api_payment>(api_payment)
            .values(&new_api_payment)
            .get_result(connection)
    }

    pub fn find_one_by_request(
        payment_request: String,
        connection: &PgConnection,
    ) -> Option<ApiPayment> {
        use crate::db::schema::api_payment::dsl::*;
        api_payment
            .filter(request.eq(payment_request))
            .first::<ApiPayment>(connection)
            .optional()
            .unwrap()
    }

    pub async fn create_from_client(
        lnd_client: LndClient,
        db: PostgresConn,
        _invoice_params: Option<InvoiceParams>,
    ) -> Result<ApiPayment, Error> {
        let client = lnd_client.0;
        let invoice =
            InvoiceUtils::generate_invoice(client, InvoiceParams::new(None, None, None)).await;

        db.run(move |c| Self::create(NewApiPayment::from(invoice), c))
            .await
    }
}
