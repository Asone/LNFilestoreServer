use crate::db::models::payment::NewPayment;
use crate::db::models::payment::Payment;
use crate::lnd::invoice::InvoiceUtils;
use crate::{db::models::Post, graphql::context::GQLContext};
use juniper::{FieldError, Value};
use uuid::Uuid;

pub struct QueryUtils {}

impl QueryUtils {
    pub async fn generate_invoiced_error(
        context: &GQLContext,
        post_id: Uuid,
        post: Post,
        message: &str,
    ) -> FieldError {
        let connection = context.get_db_connection();
        let invoice =
            InvoiceUtils::generate_post_invoice(context.get_lnd_client().clone(), post).await;
        let payment = connection
            .run(move |c| Payment::create(NewPayment::from((invoice, post_id)), c))
            .await;

        match payment {
            Ok(payment) => {
                let request = payment.request.as_str();
                let hash = payment.hash.as_str();

                FieldError::new(
                    format!("{} Use provided payment request.", message),
                    graphql_value!({"state": "open",
                         "payment_request": request, 
                         "r_hash": hash}),
                )
            }
            Err(_) => FieldError::new(
                format!(
                    "{}. An error happened while trying to generate payment request",
                    message
                ),
                Value::null(),
            ),
        }
    }
}
