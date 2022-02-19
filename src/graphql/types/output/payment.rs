use crate::db::models::payment::Payment;
use chrono::NaiveDateTime;

#[derive(GraphQLObject)]
#[graphql(description = "A payment request object")]
pub struct PaymentType {
    #[graphql(description = "The paywall ln invoice payment request string")]
    payment_request: String,
    #[graphql(description = "The expiry time of current invoice")]
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
