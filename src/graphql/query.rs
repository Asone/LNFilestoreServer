use super::types::input::post::PayablePostInput;
use super::types::output::post::PostType;
use super::types::output::post::PreviewPostType;
use crate::db::models::payment::NewPayment;
use crate::db::models::payment::Payment;
use crate::lnd::invoice::InvoiceUtils;
use crate::{
    db::models::{ Post},
    graphql::context::GQLContext,
};
use juniper::{FieldError, Value};
use tonic_lnd::rpc::{invoice::InvoiceState};
use uuid::Uuid;

pub struct Query;

#[juniper::graphql_object(context = GQLContext)]
impl Query {


    /*
        Retrieves the list of posts from DB.
        We use a specific post type here to limit the accessible information of posts through
        the request
     */
    async fn get_posts_list(context: &'a GQLContext) -> Result<Vec<PreviewPostType>,FieldError>{
        let connection = context.get_db_connection();
        let db_results = connection
            .run(move |c| Post::find_all_published(c))
            .await;

        Ok(db_results.into_iter().map(|p| PreviewPostType::from(p)).collect::<Vec<PreviewPostType>>())
        
    }

    /*
        Gets a post. 
        This is the main request where paywall shall be applied. 
     */
    async fn get_post<'a, 'b>(
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
            Some(r) => match r.published { // Checks if post is published
                true =>  match r.is_payable() { // Checks if there should be a paywall ( price > 0 )
                    true => match post.payment_request { // If payable, ensure there's a payment_request provided
                        Some(payment_request) => { // payment_request found

                            // Search for payment entry based on the payment_request provided
                            let payment = connection
                                .run(move |c| Payment::find_one_by_request(payment_request.clone(), c))
                                .await;
                            match payment {
                                Some(payment) => { // Payment found

                                    // Request LND invoice and checks the invoice state
                                    match InvoiceUtils::get_invoice_state_from_payment_request(context, payment.request).await {
                                        Ok(invoice_result) => match invoice_result {
                                            Some(invoice) => match invoice.state() {
                                                InvoiceState::Settled => Ok(PostType::from(r)), // Payment has been done. Serves the post
                                                InvoiceState::Open => Err(FieldError::new(
                                                    "Awaiting for payment to be done.",
                                                    Value::null(),
                                                )), // Payment hasn't been done yet. We shall wait for payment, so there's no need to regenerate an invoice
                                                InvoiceState::Accepted => Err(FieldError::new(
                                                    "Payment ongoing but not settled yet",
                                                    Value::null(),
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
                        None => Err(QueryUtils::generate_invoiced_error(context,post_id,r,"Payable post. Payment not found.").await)
                        
                    },
                    false => Ok(PostType::from(r)), // Post has a price of 0 (free), so we serve it without condition
                },
                false => Err(FieldError::new("Post not found", Value::Null)), // Post not published
            }
            // Post has not been found in DB
            None => Err(FieldError::new("Post not found", Value::Null)),
        }
    }
}


pub struct QueryUtils{}

impl QueryUtils {

    pub async fn generate_invoiced_error(context: &GQLContext,post_id: Uuid, post: Post, message: &str ) -> FieldError {
        let connection = context.get_db_connection();
        let invoice = InvoiceUtils::generate_invoice(context, post).await;
        let payment = connection.run(move |c| Payment::create(NewPayment::from((invoice,post_id)), c)).await;

        match payment {
            Ok(payment) => { 

                let request = payment.request.as_str();
                let hash = payment.hash.as_str();

                FieldError::new(
                    format!("{} Use provided payment request.",message),
                    graphql_value!({"payment_request": request, "r_hash": hash})
                )
            },
            Err(_) => {
                FieldError::new(
                    format!("{}. An error happened while trying to generate payment request",message),
                    Value::null(),
                )
            }
        }
    }
}