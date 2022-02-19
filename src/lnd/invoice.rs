use crate::db::models::Post;
use chrono::{Duration, NaiveDateTime, Utc};
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::{Code, Status};
use tonic_lnd::rpc::lightning_client::LightningClient;
use tonic_lnd::rpc::{Invoice, PaymentHash};
use tonic_lnd::MacaroonInterceptor;
use std::env;

use lightning_invoice::*;
extern crate dotenv;

pub struct InvoiceParams {
    pub value: i64,
    pub memo: String,
    pub expiry: i64,
}

impl InvoiceParams {
    pub fn new(value: Option<i64>, memo: Option<String>, expiry: Option<i64>) -> Self {
        
        let default_value = env::var("DEFAULT_INVOICE_VALUE").unwrap_or("100".to_string());
        let default_expiry = env::var("DEFAULT_INVOICE_EXPIRY").unwrap_or("API Payment".to_string());
        let default_memo = env::var("DEFAULT_INVOICE_MEMO").unwrap_or("600".to_string());

        Self {
            value: value.unwrap_or_else(|| default_value.parse::<i64>().unwrap()),
            memo: memo.unwrap_or_else(|| default_memo),
            expiry: expiry.unwrap_or_else(|| default_expiry.parse::<i64>().unwrap()),
        }
    }
}

/*
   Represents a simplified invoice object.
   This allow us to keep only the critical data we need from an
   invoice object
*/
#[derive(Clone)]
pub struct LndInvoice {
    pub memo: String,
    pub payment_request: String,
    pub value: i64,
    pub r_hash: String,
    pub expires_at: NaiveDateTime,
}

impl LndInvoice {
    /*
      Creates an LndInvoice from an Invoice retrieved through RPC + its rHash
    */
    pub fn new(invoice: tonic_lnd::rpc::Invoice, r_hash: String) -> Self {
        let expires_at = Utc::now()
            .checked_add_signed(Duration::seconds(invoice.expiry))
            .unwrap();
        Self {
            payment_request: invoice.payment_request,
            memo: invoice.memo,
            value: invoice.value as i64,
            r_hash: r_hash,
            expires_at: expires_at.naive_utc(),
        }
    }
}

/**
 * Provides with the utilities method required to build the Lightning Network
 * Paywall on top of the Juniper GraphQL API.
 */
pub struct InvoiceUtils {}

impl InvoiceUtils {
    /**
        Generates an invoice for a post
        This shall be called whenever the user
        requests a resource without providing a payment request value
        or when the related invoice is expired/canceled.
    */
    pub async fn generate_post_invoice(
        lnd_client: LightningClient<
            InterceptedService<tonic::transport::Channel, MacaroonInterceptor>,
        >,
        post: Post,
    ) -> LndInvoice {
        let params = InvoiceParams::new(
            Some(post.price as i64),
            // Memo content should be handle with an env var pattern
            Some(format!("buy {} : {}", post.uuid, post.title).to_string()),
            Some(60 as i64),
        );
        // Request invoice generation to the LN Server
        InvoiceUtils::generate_invoice(lnd_client, params).await
    }

    /**
       Generate an invoice through lnd
    */
    pub async fn generate_invoice(
        mut lnd_client: LightningClient<
            InterceptedService<tonic::transport::Channel, MacaroonInterceptor>,
        >,
        params: InvoiceParams,
    ) -> LndInvoice {
        let add_invoice_response = lnd_client.add_invoice(tonic_lnd::rpc::Invoice {
            memo: params.memo,
            value: params.value,
            expiry: params.expiry,
            ..tonic_lnd::rpc::Invoice::default()
        });

        let result = add_invoice_response.await.unwrap().into_inner();

        // Retrieve the payment hash based on r_hash returned from the AddInvoiceResponse
        let payment_hash = tonic_lnd::rpc::PaymentHash {
            r_hash: result.r_hash.clone(),
            r_hash_str: hex::encode(result.r_hash.clone()), // provided as request by the Struct but not used and deprecated
        };

        // // Get the Invoice detail so we can return the payment_request
        let invoice = lnd_client
            .lookup_invoice(payment_hash)
            .await
            .unwrap()
            .into_inner();

        LndInvoice::new(invoice, hex::encode(result.r_hash))
    }

    /*
        Gets the invoice state from a payment request string.
        It consists as a two steps method.

        First it registers an invoice
    */
    pub async fn get_invoice_state_from_payment_request<'a>(
        lnd_client: &LightningClient<
            InterceptedService<tonic::transport::Channel, MacaroonInterceptor>,
        >,
        payment_request: String,
    ) -> Result<Option<Invoice>, Status> {
        let mut client = lnd_client.clone();

        // Parse the payment request
        let invoice = payment_request
            .as_str()
            .parse::<SignedRawInvoice>()
            .unwrap();

        // Get the payment hash
        let p_hash = invoice.payment_hash().unwrap();

        /*
            The below instruction might seems a bit odd.
            the expected r_hash here is not the Invoice r_hash
            but rather the r_hash of the payment request which is
            denominated in the SignedRawInvoice as the payment_hash.
        */
        let request = tonic::Request::new(PaymentHash {
            r_hash: p_hash.0.to_vec(),
            ..PaymentHash::default()
        });

        match client.lookup_invoice(request).await {
            Ok(response) => Ok(Some(response.into_inner())),
            Err(status) => {
                if status.code() == Code::Unknown
                    && (status.message() == "there are no existing invoices"
                        || status.message() == "unable to locate invoice")
                {
                    Ok(None)
                } else {
                    Err(status)
                }
            }
        }
    }
}
