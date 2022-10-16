use crate::db::models::media_payment::MediaPayment;
use chrono::NaiveDateTime;
use tonic_lnd::rpc::invoice::InvoiceState;

#[derive(GraphQLObject)]
#[graphql(description = "A payment request object")]
pub struct PaymentType {
    #[graphql(description = "The paywall ln invoice payment request string")]
    payment_request: String,
    #[graphql(description = "The expiry time of current invoice")]
    expires_at: NaiveDateTime,
    #[graphql(description = "The current state of the payment request")]
    state: Option<String>,
}

impl From<MediaPayment> for PaymentType {
    fn from(item: MediaPayment) -> Self {
        Self {
            payment_request: item.request,
            expires_at: item.expires_at,
            state: None,
        }
    }
}

impl From<(MediaPayment, InvoiceState)> for PaymentType {
    fn from(item: (MediaPayment, InvoiceState)) -> Self {
        Self {
            payment_request: item.0.request,
            expires_at: item.0.expires_at,
            state: Some(Self::state_from_invoice_state(item.1)),
        }
    }
}

impl From<(MediaPayment, &InvoiceState)> for PaymentType {
    fn from(item: (MediaPayment, &InvoiceState)) -> Self {
        Self {
            payment_request: item.0.request,
            expires_at: item.0.expires_at,
            state: Some(Self::state_from_invoice_state(*item.1)),
        }
    }
}

impl PaymentType {
    pub fn state_from_invoice_state(invoice_state: InvoiceState) -> String {
        match invoice_state {
            InvoiceState::Accepted => String::from("accepted"),
            InvoiceState::Canceled => String::from("canceled"),
            InvoiceState::Settled => String::from("settled"),
            InvoiceState::Open => String::from("open"),
        }
    }
}
