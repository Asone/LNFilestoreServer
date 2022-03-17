/*!

# juniper_rocket

This repository contains the [Rocket][Rocket] web server integration for
[Juniper][Juniper], a [GraphQL][GraphQL] implementation for Rust.

## Documentation

For documentation, including guides and examples, check out [Juniper][Juniper].

A basic usage example can also be found in the [Api documentation][documentation].

## Examples

Check [examples/rocket_server.rs][example] for example code of a working Rocket
server with GraphQL handlers.

## Links

* [Juniper][Juniper]
* [Api Reference][documentation]
* [Rocket][Rocket]

## License

This project is under the BSD-2 license.

Check the LICENSE file for details.

[Rocket]: https://rocket.rs
[Juniper]: https://github.com/graphql-rust/juniper
[GraphQL]: http://graphql.org
[documentation]: https://docs.rs/juniper_rocket
[example]: https://github.com/graphql-rust/juniper_rocket/blob/master/examples/rocket_server.rs

*/

#![doc(html_root_url = "https://docs.rs/juniper_rocket/0.7.1")]

use std::{borrow::Cow, io::Cursor, sync::Arc, collections::HashMap};

use juniper_rocket::{GraphQLResponse, GraphQLContext};
use multer::Multipart;
use rocket::{
    data::{self, FromData, ToByteUnit},
    form::{error::ErrorKind, DataField, Error, Errors, FromForm, Options, ValueField},
    http::{ContentType, Status},
    outcome::Outcome::{Failure, Forward, Success},
    response::{self, content, Responder, Response},
    Data, Request,
};

use juniper::{
    http::{self, GraphQLBatchRequest},
    DefaultScalarValue, FieldError, GraphQLSubscriptionType, GraphQLType, GraphQLTypeAsync,
    InputValue, RootNode, ScalarValue,
};

enum FormType{
    IS_JSON,
    GRAPHQL,
    MULTIPART
}

/// Simple wrapper around an incoming GraphQL request
///
/// See the `http` module for more information. This type can be constructed
/// automatically from both GET and POST routes by implementing the `FromForm`
/// and `FromData` traits.
#[derive(Debug, PartialEq)]
pub struct GraphQLUploadRequest<S = DefaultScalarValue>(GraphQLBatchRequest<S>,Option<Vec<u8>>)
where
    S: ScalarValue;

/// Simple wrapper around the result of executing a GraphQL query
// pub struct GraphQLResponse(pub Status, pub String);


impl<S> GraphQLUploadRequest<S>
where
    S: ScalarValue,
{
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

    /// Returns the operation names associated with this request.
    ///
    /// For batch requests there will be multiple names.
    pub fn operation_names(&self) -> Vec<Option<&str>> {
        self.0.operation_names()
    }
}

const BODY_LIMIT: u64 = 1024 * 100;

#[rocket::async_trait]
impl<'r, S> FromData<'r> for GraphQLUploadRequest<S>
where
    S: ScalarValue,
{
    type Error = String;

    async fn from_data(
        req: &'r Request<'_>,
        data: Data<'r>,
    ) -> data::Outcome<'r, Self, Self::Error> {

        // Get content-type of HTTP request
        let content_type = req.content_type().unwrap();
        
        // Split content-type value as a tuple of str
        let content_type_value = (content_type.top().as_str(),content_type.sub().as_str());
        
        // Identify the value to aknowledge which kind of parsing action we
        // need to provide
        let content_type_enum_value = match content_type_value {
            ("application", "json") => FormType::IS_JSON,
            ("application", "graphql") => FormType::GRAPHQL,
            ("multipart","form-data") => FormType::MULTIPART,
            _ => return Box::pin(async move { Forward(data) }).await,
        };
        
        Box::pin(async move {
   
            Success(GraphQLUploadRequest({

                match content_type_enum_value {
                    FormType::IS_JSON => { // Content-type is declared as json
                        let mut body = String::new();
                        let mut reader = data.open(BODY_LIMIT.bytes());
            
                        match serde_json::from_str(&body) {
                            Ok(req) => req,
                            Err(e) => return Failure((Status::BadRequest, format!("{}", e))),
                        }
                    },
                    FormType::GRAPHQL => { // Content-type is declared as graphQL 
                        let mut body = String::new();
                        let mut reader = data.open(BODY_LIMIT.bytes());
            
                        GraphQLBatchRequest::Single(http::GraphQLRequest::new(body, None, None))
                    },
                    FormType::MULTIPART => { // Content-type is declared as a multipart request
                        let mut query: String = "".to_string();
                        // Get the boundary attached to the multipart form
                        let (_, boundary) = match content_type.params().find(|&(k, _)| k == "boundary") {
                            Some(s) => s,
                            None => return Failure((Status::InternalServerError, format!("An error happened"))),
                        };
                        
                        // Reads the datastream of request and converts it to a multipart object
                        let mut body = String::new();
                        let mut reader = data.open(BODY_LIMIT.bytes());
                        let stream = tokio_util::io::ReaderStream::new(reader);
                        let mut multipart = Multipart::new(stream, boundary);

                        // Iterate through the different fields of the data-form
                        while let Some(entry) = multipart.next_field().await.unwrap() {
                                let field_name = match entry.name() {
                                    Some(name) => {
                                    println!("{}",name);
                                    let m = Arc::<&str>::from(name);
                                    m
                                    },
                                    None => continue,
                                };

                                let name = field_name.to_string();
                                // Check if there is some mimetype which should exist when a field is a binary file
                                if let Some(content_type) = entry.content_type().as_ref() {
                                    let top = content_type.type_();
                                    let sub = content_type.subtype();

                                    let content = entry.bytes().await;
                                } else { // No mimetype so we expect this to not be a file but rather a json content
                                    let r = entry.text().await;

                                    match r {
                                        Ok(result) => {
                                            // println!("{}",field_name);
                                            if name.as_str() == "operations" {
                                               query = result;
                                            }
                                        },
                                        Err(_) => {}
                                    }
                                    // println!("field : {}, with no MimeType",field_name);
                                }
                        }

                        match serde_json::from_str(&query) {
                            Ok(req) => req,
                            Err(e) => return Failure((Status::BadRequest, format!("{}", e))),
                        }

                        // GraphQLBatchRequest::Single(http::GraphQLRequest::new(query, None, None))
                    }
                }
            
            },None))
        })
        .await
      
    }
}
