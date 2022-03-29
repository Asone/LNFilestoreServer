use juniper::{GraphQLTypeAsync, GraphQLType};
use juniper_rocket::{GraphQLResponse};
use rocket::http::Status;
use juniper::{
    http::GraphQLBatchRequest,
    DefaultScalarValue, GraphQLSubscriptionType,
    RootNode, ScalarValue,
};

/// A GraphQL operations request.
/// 
/// This struct replicates the [`GraphQLRequest`](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/src/lib.rs#L64) original behavior.
/// It is provided and used with the upload wrapper as the original struct
/// does not provide any constructor and the tuple constructor is private.
#[derive(Debug, PartialEq)]
pub struct GraphQLOperationsRequest<S = DefaultScalarValue>(
pub GraphQLBatchRequest<S>
)
where
    S: ScalarValue;


impl<S> GraphQLOperationsRequest<S>
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
        let response = self.0.execute(root_node, context).await;
        let status = if response.is_ok() {
            Status::Ok
        } else {
            Status::BadRequest
        };
        let json = serde_json::to_string(&response).unwrap();

        GraphQLResponse(status, json)
    }

    /// Synchronously execute an incoming GraphQL query.
    pub fn execute_sync<CtxT, QueryT, MutationT, SubscriptionT>(
        &self,
        root_node: &RootNode<QueryT, MutationT, SubscriptionT, S>,
        context: &CtxT,
    ) -> GraphQLResponse
    where
        QueryT: GraphQLType<S, Context = CtxT>,
        MutationT: GraphQLType<S, Context = CtxT>,
        SubscriptionT: GraphQLType<S, Context = CtxT>,
    {
        let response = self.0.execute_sync(root_node, context);
        let status = if response.is_ok() {
            Status::Ok
        } else {
            Status::BadRequest
        };
        let json = serde_json::to_string(&response).unwrap();

        GraphQLResponse(status, json)
    }

    /// Returns the operation names associated with this request.
    ///
    /// For batch requests there will be multiple names.
    pub fn operation_names(&self) -> Vec<Option<&str>> {
        self.0.operation_names()
    }
}
