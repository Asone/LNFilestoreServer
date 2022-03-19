/*
# GraphQL Upload Request handler for rocket_juniper
#
*/

use juniper::GraphQLTypeAsync;
use juniper_rocket::{GraphQLResponse};
use multer::Multipart;
use rocket::{
    data::{self, FromData, ToByteUnit},
    form::{Error},
    http::{ContentType, Status},
    outcome::Outcome::{Failure, Forward, Success},
    Data, Request,
};
use std::fs::File;
use std::io::prelude::*;
use std::{sync::Arc};

use juniper::{
    http::{self, GraphQLBatchRequest},
    DefaultScalarValue, GraphQLSubscriptionType,
    RootNode, ScalarValue,
};
use serde::Deserialize;

#[derive(Debug)]
pub enum MultipartFormParsingError {
    BoundaryParsingError
}

enum ProcessorType {
    JSON,
    GRAPHQL,
    MULTIPART,
    UKNOWN,
}

// This shall be deferable to env
const BODY_LIMIT: u64 = 1024 * 100;

#[derive(Debug, PartialEq, Deserialize)]
pub struct GraphQLUploadRequest<S = DefaultScalarValue>
where
    S: ScalarValue,
{
    pub gql_request: GraphQLBatchRequest<S>,
    pub files: Option<Vec<String>>,
}

impl<S> GraphQLUploadRequest<S>
where
    S: ScalarValue,
{
    /**
       Body reader for application/json content type.
       This method replicates the original handler from juniper_rocket
    */
    fn from_json_body<'r>(data: Data<'r>) -> Result<(GraphQLBatchRequest<S>,Option<Vec<String>>), serde_json::Error> {
        let body = String::new();
        let mut _reader = data.open(BODY_LIMIT.bytes());

        match serde_json::from_str(&body) {
            Ok(req) => Ok(req),
            Err(e) => Err(e),
        }
    }

    /**
       Body reader for application/graphql content type.
       This method replicates the original handler from juniper_rocket
    */
    fn from_graphql_body<'r>(data: Data<'r>) -> Result<GraphQLBatchRequest<S>, Error> {
        let body = String::new();
        let mut _reader = data.open(BODY_LIMIT.bytes());

        Ok(GraphQLBatchRequest::Single(http::GraphQLRequest::new(
            body, None, None,
        )))
    }

    /**
       Body reader for multipart/form-data content type.
    */
    async fn from_multipart_body<'r>(
        data: Data<'r>,
        content_type: &ContentType,
    ) -> Result<(GraphQLBatchRequest<S>,Option<Vec<String>>), Error<'r>> {
        // Builds a void query for development
        let mut query: String = String::new();
        let boundary = Self::get_boundary(content_type).unwrap();

        // Create and read a datastream from the request body
        let reader = data.open(BODY_LIMIT.bytes());
        let stream = tokio_util::io::ReaderStream::new(reader);

        // Create a multipart object based on multer
        let mut multipart = Multipart::new(stream, boundary);
        let mut files = Vec::<String>::new();

        // Iterate on the form fields, which can be
        // either text content or binary.
        while let Some(entry) = multipart.next_field().await.unwrap() {
            let field_name = match entry.name() {
                Some(name) => Arc::<&str>::from(name),
                None => continue,
            };

            let name = field_name.to_string();
           
            // Check if there is some mimetype which should exist when a field is a binary file
            if let Some(_mime_type) = entry.content_type().as_ref() {

                let file_name = match entry.file_name() {
                    Some(filename) => filename,
                    None => continue,
                };

                let path = format!("./tmp/{}", file_name);
                let content = entry.bytes().await.unwrap();

                let mut file = File::create(&path).unwrap();
                file.write_all(&content);
                files.push(path);

            } else {
                // No mimetype so we expect this to not be a file but rather a json content
                let r = entry.text().await;

                // If field name is operations which should be the graphQL Query
                // according to spec
                match r {
                    Ok(result) => {
                        if name.as_str() == "operations" {
                            query = result;
                        }
                    }
                    Err(_) => {}
                };
            }
        }

        // Default parser
        match serde_json::from_str(&query) {
            Ok(req) => Ok((req,Some(files))),
            Err(_) => Err(rocket::form::Error::validation("The provided request could not be parsed.")),
        }
    }

    /**
       Returns an enum value for a specific processors based on request Content-type
    */
    fn get_processor_type(content_type: &ContentType) -> ProcessorType {
        let top = content_type.top().as_str();
        let sub = content_type.sub().as_str();

        match (top, sub) {
            ("application", "json") => ProcessorType::JSON,
            ("application", "graphql") => ProcessorType::GRAPHQL,
            ("multipart", "form-data") => ProcessorType::MULTIPART,
            _ => ProcessorType::UKNOWN,
        }
    }

    /**
       Extracts the boundary for a multipart/form-data request
    */
    pub fn get_boundary(content_type: &ContentType) -> Result<&str, MultipartFormParsingError> {
        match content_type.params().find(|&(k, _)| k == "boundary") {
            Some(s) => Ok(s.1),
            None => Err(MultipartFormParsingError::BoundaryParsingError),
        }
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

        match Self::get_processor_type(content_type) {
            ProcessorType::JSON => {
                Box::pin(async move {
                    match Self::from_json_body(data) {
                        Ok(result) => Success(GraphQLUploadRequest {
                            gql_request: result.0,
                            files: result.1,
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                    // Success(Self {})
                })
                .await
            }
            ProcessorType::GRAPHQL => {
                Box::pin(async move {
                    match Self::from_graphql_body(data) {
                        Ok(result) => Success(GraphQLUploadRequest {
                            gql_request: result,
                            files: None,
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                })
                .await
            }
            ProcessorType::MULTIPART => {
                Box::pin(async move {
                    match Self::from_multipart_body(data, content_type).await {
                        Ok(result) => Success(GraphQLUploadRequest {
                            gql_request: result.0,
                            files: result.1
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                })
                .await
            }
            ProcessorType::UKNOWN => Box::pin(async move { Forward(data) }).await,
        }
    }
}
