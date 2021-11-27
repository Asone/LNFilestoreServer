use crate::{db::PostgresConn, lnd::client::LndClient};

use derive_more::Deref;
use tonic::transport::Channel;
use tonic_lnd::rpc::lightning_client::LightningClient;

/*
   The GQLContext struct provides an extended juniper context
   with references to instances for both the database and the
   lightning network client.
   This allow us :
    - To interact with the database pool
    - To interact with the lightning network server
*/
#[derive(Deref)]
pub struct GQLContext {
    #[deref]
    pub pool: PostgresConn,
    pub lnd: LndClient,
}

impl juniper::Context for GQLContext {}

impl AsRef<Self> for GQLContext {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl GQLContext {
    // Provides the instance of the LN Client
    pub fn get_lnd_client(&self) -> &LightningClient<Channel> {
        return &self.lnd.0;
    }

    // Provides the instance of DB pool
    pub fn get_db_connection(&self) -> &PostgresConn {
        return &self.pool;
    }
}
