use chrono::NaiveDateTime;
use tonic_lnd::rpc::invoice::InvoiceState;
use uuid::Uuid;

use crate::db::models::media_payment::MediaPayment;

#[derive(GraphQLObject)]
pub struct AvailablePayment {
    #[graphql(description = "The related media uuid")]
    media_uuid: Uuid,
    #[graphql(description = "The paywall ln invoice payment request string")]
    payment_request: String,
    #[graphql(description = "The expiry time of current invoice")]
    expires_at: NaiveDateTime,
    #[graphql(description = "The current state of the payment request")]
    state: Option<String>,
}

#[derive(GraphQLObject)]
pub struct ReplacementPayment {
    #[graphql(description = "The related media uuid")]
    media_uuid: Uuid,
    #[graphql(description = "The paywall ln invoice payment request string")]
    payment_request: String,
    #[graphql(description = "The expiry time of current invoice")]
    expires_at: NaiveDateTime,
    #[graphql(description = "The current state of the payment request")]
    state: Option<String>,
}

#[derive(GraphQLObject)]
pub struct SettledPayment {
    #[graphql(description = "The related media uuid")]
    media_uuid: Uuid,
    #[graphql(description = "The paywall ln invoice payment request string")]
    payment_request: String,
    #[graphql(description = "The current state of the payment request")]
    state: Option<String>,
}

#[derive(GraphQLUnion)]
pub enum MediaInvoice {
    ReplacementPayment(ReplacementPayment),
    AvailablePayment(AvailablePayment),
    SettledPayment(SettledPayment),
}

impl From<(MediaPayment, InvoiceState)> for MediaInvoice {
    fn from(data: (MediaPayment, InvoiceState)) -> Self {
        match data.1 {
            InvoiceState::Accepted => Self::AvailablePayment(AvailablePayment {
                media_uuid: data.0.media_uuid,
                payment_request: data.0.request,
                expires_at: data.0.expires_at,
                state: Some("accepted".to_string()),
            }),
            InvoiceState::Open => Self::AvailablePayment(AvailablePayment {
                media_uuid: data.0.media_uuid,
                payment_request: data.0.request,
                expires_at: data.0.expires_at,
                state: Some("open".to_string()),
            }),
            InvoiceState::Settled => Self::SettledPayment(SettledPayment {
                media_uuid: data.0.media_uuid,
                payment_request: data.0.request,
                state: Some("settled".to_string()),
            }),
            InvoiceState::Canceled => Self::ReplacementPayment(ReplacementPayment {
                media_uuid: data.0.media_uuid,
                payment_request: data.0.request,
                expires_at: data.0.expires_at,
                state: Some("open".to_string()),
            }),
        }
    }
}
