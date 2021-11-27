use crate::db::models::payment::{self, Payment};
use chrono::NaiveDateTime;

#[derive(GraphQLObject)]
#[graphql(description = "A payment request object")]
pub struct PaymentType {
    payment_request: String,
    expires_at: NaiveDateTime,
}

impl From<Payment> for PaymentType {
    fn from(item: Payment) -> Self {
        Self {
            payment_request: item.request,
            expires_at: item.expires_at,
        }
    }
}
