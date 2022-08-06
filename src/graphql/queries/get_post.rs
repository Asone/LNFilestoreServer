use super::utils::QueryUtils;
use crate::db::models::payment::Payment;
use crate::graphql::types::input::post::PayablePostInput;
use crate::graphql::types::output::post::PostType;
use crate::lnd::invoice::InvoiceUtils;
use crate::{db::models::Post, graphql::context::GQLContext};
use juniper::{FieldError, Value};
use tonic_lnd::rpc::invoice::InvoiceState;

pub async fn get_post<'a, 'b>(
    context: &'a GQLContext,
    post: PayablePostInput,
) -> Result<PostType, FieldError> {
    let post_id: uuid::Uuid = post.uuid.clone();
    let connection = context.get_db_connection();

    // Find the post in the database
    let result = connection
        .run(move |c| Post::find_one_by_id(post_id, c))
        .await;

    match result {
        Some(r) => match r.published {
            // Checks if post is published
            true => match r.is_payable() {
                // Checks if there should be a paywall ( price > 0 )
                true => match post.payment_request {
                    // If payable, ensure there's a payment_request provided
                    Some(payment_request) => {
                        // payment_request found

                        // Search for payment entry based on the payment_request provided
                        let payment = connection
                            .run(move |c| Payment::find_one_by_request(payment_request.clone(), c))
                            .await;
                        match payment {
                            Some(payment) => { // Payment found

                                // Request LND invoice and checks the invoice state
                                match InvoiceUtils::get_invoice_state_from_payment_request(context.get_lnd_client(), payment.request).await {
                                        Ok(invoice_result) => match invoice_result {
                                            Some(invoice) => match invoice.state() {
                                                InvoiceState::Settled => Ok(PostType::from(r)), // Payment has been done. Serves the post
                                                InvoiceState::Open => Err(FieldError::new(
                                                    "Awaiting for payment to be done.",
                                                    graphql_value!({"state": "open"}),
                                                )), // Payment hasn't been done yet. We shall wait for payment, so there's no need to regenerate an invoice
                                                InvoiceState::Accepted => Err(FieldError::new(
                                                    "Payment ongoing but not settled yet",
                                                    graphql_value!({"state": "ongoing"}),
                                            )), // Payment is on process onto the network but has not reach its receiver yet. We shall wait, so there's no need to regenerate an invoice
                                            InvoiceState::Canceled => Err(QueryUtils::generate_invoiced_error(context,post_id,r,"Payment expired or canceled.").await),
                                        },
                                        // LND Server says there's no invoice matching
                                        None => Err(QueryUtils::generate_invoiced_error(context,post_id,r,"No invoice found for corresponding payment request. Proceed to payment with the provided request payment").await)

                                    },
                                    // Invoice is broken. Maybe we should serve a new invoice here ?
                                    Err(_) => Err(FieldError::new(
                                        "An error happened when trying to decode invoice",
                                        Value::null(),
                                    )),
                                }
                            },
                            // Our DB does not contain any payment with the provided payment_request.
                            None => Err(QueryUtils::generate_invoiced_error(context,post_id,r,"No recorded payment request related to the requested post found with the payment requested provided.").await)                            
                        }
                    }
                    None => Err(QueryUtils::generate_invoiced_error(
                        context,
                        post_id,
                        r,
                        "Payable post. Payment not found.",
                    )
                    .await),
                },
                false => Ok(PostType::from(r)), // Post has a price of 0 (free), so we serve it without condition
            },
            false => Err(FieldError::new("Post not found", Value::Null)), // Post not published
        },
        // Post has not been found in DB
        None => Err(FieldError::new("Post not found", Value::Null)),
    }
}
