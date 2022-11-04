use std::collections::HashMap;

use crate::{
    db::{
        models::user::{User, UserRoleEnum},
        PostgresConn,
    },
    lnd::client::LndClient,
};

use derive_more::Deref;
use juniper_rocket_multipart_handler::temp_file::TempFile;
// use tonic::codegen::InterceptedService;
use tonic_lnd::tonic::codegen::InterceptedService;
use tonic_lnd::{rpc::lightning_client::LightningClient, MacaroonInterceptor};

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
    pub files: Option<HashMap<String, TempFile>>,
    pub user: Option<User>,
    pub server_config: Option<String>,
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
    pub fn get_lnd_client(
        &self,
    ) -> &LightningClient<InterceptedService<tonic::transport::Channel, MacaroonInterceptor>> {
        return &self.lnd.0;
    }

    // Provides the instance of DB pool
    pub fn get_db_connection(&self) -> &PostgresConn {
        return &self.pool;
    }

    // Provides files collection that have been sent through request
    pub fn get_files(&self) -> &Option<HashMap<String, TempFile>> {
        return &self.files;
    }

    /// Provides the instance of optional user
    pub fn get_user(&self) -> &Option<User> {
        return &self.user;
    }

    // Checks if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        match &self.user {
            Some(_) => true,
            None => false,
        }
    }

    // Checks if user is granted with role from a list of roles
    pub fn has_permissioned_role(&self, roles: Vec<UserRoleEnum>) -> bool {
        match &self.user {
            Some(user) => roles.contains(&user.role),
            None => false,
        }
    }
}
