use crate::db::models::payment::NewPayment;
use crate::db::models::payment::Payment;
use crate::graphql::types::output::payment::PaymentType;
use crate::lnd::invoice::InvoiceUtils;
use crate::{db::models::Post, graphql::context::GQLContext};
use juniper::{FieldError, Value};

pub async fn request_invoice_for_post<'a>(
    context: &'a GQLContext,
    post_id: uuid::Uuid,
) -> Result<PaymentType, FieldError> {
    let connection = context.get_db_connection();
    let db_result = connection
        .run(move |c| Post::find_one_by_id(post_id, c))
        .await;

    match db_result {
        Some(post) => {
            let invoice =
                InvoiceUtils::generate_post_invoice(context.get_lnd_client().clone(), post).await;
            let payment = connection
                .run(move |c| Payment::create(NewPayment::from((invoice, post_id)), c))
                .await;

            match payment {
                Ok(payment) => Ok(PaymentType::from(payment)),
                Err(_) => Err(FieldError::new(
                    "Could not find post with provided uuid",
                    Value::Null,
                )),
            }
        }
        None => Err(FieldError::new(
            "Could not find post with provided uuid",
            Value::Null,
        )),
    }
}
