use crate::{db::PostgresConn, lnd::client::LndClient};

use derive_more::Deref;
use tonic::transport::Channel;
use tonic_lnd::rpc::lightning_client::LightningClient;

/*
   The GQLPayableContext struct provides an extended juniper context
   with references to instances for both the database and the
   lightning network client.
   This allow us :
    - To interact with the database pool
    - To interact with the lightning network server
*/
#[derive(Deref)]
pub struct GQLPayableContext {
    #[deref]
    pub pool: PostgresConn,
    pub lnd: LndClient,
    pub payment_request: String,
}

impl juniper::Context for GQLPayableContext {}

impl AsRef<Self> for GQLPayableContext {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl GQLPayableContext {
    // Provides the instance of the LN Client
    pub fn get_lnd_client(&self) -> &LightningClient<Channel> {
        return &self.lnd.0;
    }

    // Provides the instance of DB pool
    pub fn get_db_connection(&self) -> &PostgresConn {
        return &self.pool;
    }

    pub fn get_payment_request(&self) -> &String {
        return &self.payment_request;
    }
}
