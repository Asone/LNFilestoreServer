use juniper::{GraphQLTypeAsync};
use juniper_rocket::{GraphQLResponse};
use multer::{Multipart, bytes::Bytes, Field};
use rocket::{
    data::{self, FromData, ToByteUnit},
    form::{Error},
    http::{ContentType, Status},
    outcome::Outcome::{Failure, Forward, Success},
    Data, Request, fs::FileName, Either,
};
use std::{fs::File, env, num::NonZeroU64, path::PathBuf, collections::HashMap};
use std::io::prelude::*;
use std::{sync::Arc};

use juniper::{
    http::{self, GraphQLBatchRequest},
    DefaultScalarValue, GraphQLSubscriptionType,
    RootNode, ScalarValue,
};
use serde::Deserialize;
#[derive(Debug, PartialEq)]
pub struct GraphQLUploadOperationsRequest<S = DefaultScalarValue>
where
    S: ScalarValue,
{
    pub gql_request: GraphQLBatchRequest<S>
}

impl<S> GraphQLUploadOperationsRequest<S>
where
    S: ScalarValue,
{
    /// Asynchronously execute an incoming GraphQL query.
    pub async fn execute<CtxT, QueryT, MutationT, SubscriptionT>(
        &self,
        root_node: &RootNode<'_, QueryT, MutationT, SubscriptionT, S>,
        context: &CtxT,
    ) -> GraphQLResponse
    where
        QueryT: GraphQLTypeAsync<S, Context = CtxT>,
        QueryT::TypeInfo: Sync,
        MutationT: GraphQLTypeAsync<S, Context = CtxT>,
        MutationT::TypeInfo: Sync,
        SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT>,
        SubscriptionT::TypeInfo: Sync,
        CtxT: Sync,
        S: Send + Sync,
    {
        let response = self.gql_request.execute(root_node, context).await;
        let status = if response.is_ok() {
            Status::Ok
        } else {
            Status::BadRequest
        };
        let json = serde_json::to_string(&response).unwrap();

        GraphQLResponse(status, json)
    }
}
