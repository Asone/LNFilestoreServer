use juniper::GraphQLTypeAsync;
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
use crate::graphql_upload_operations_request::GraphQLUploadOperationsRequest;

use crate::temp_file::TempFile;

// This shall be deferable to env
const BODY_LIMIT: u64 = 1024 * 100;

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


#[derive(Debug, PartialEq)]
pub struct GraphQLUploadWrapper<S = DefaultScalarValue>
where
    S: ScalarValue,
{
    pub operations: GraphQLUploadOperationsRequest<S>,
    pub files: Option<HashMap<String, TempFile>>,
}

impl<S> GraphQLUploadWrapper<S>
where
    S: ScalarValue,
{

    pub async fn get_files(&self) -> &Option<HashMap<String, TempFile>> {
        &self.files
    }

    /**
       Body reader for application/json content type.
       This method replicates the original handler from juniper_rocket
    */
    fn from_json_body<'r>(data: Data<'r>) -> Result<GraphQLBatchRequest<S>, serde_json::Error> {
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
    ) -> Result<(GraphQLBatchRequest<S>,Option<HashMap<String, TempFile>>), Error<'r>> {
        // Builds a void query for development
        let mut query: String = String::new();
        let boundary = Self::get_boundary(content_type).unwrap();

        // Create and read a datastream from the request body
        let reader = data.open(BODY_LIMIT.bytes());
        let stream = tokio_util::io::ReaderStream::new(reader);

        // Create a multipart object based on multer
        let mut multipart = Multipart::new(stream, boundary);
        
        let mut filesMap: HashMap<String, TempFile> = HashMap::new();
        
        // Iterate on the form fields, which can be
        // either text content or binary.
        while let Some(entry) = multipart.next_field().await.unwrap() {
            let field_name = match entry.name() {
                Some(name) => Arc::<&str>::from(name).to_string(),
                None => continue,
            };

            let name = field_name;


            let path = format!("{}",env::temp_dir().display());

            match entry.content_type().as_ref() {
                Some(mimetype) => {

                    let file_name = match entry.file_name() {
                        Some(filename) => Arc::<&str>::from(filename).to_string(),
                        None => continue,
                    };

                    let content = match entry.bytes().await {
                        Ok(d) => d,
                        Err(e) => {
                            Bytes::new()
                        }
                    };

                    let tmpfile = TempFile {
                        name: file_name,
                        size: Some(content.len()),
                        local_path: PathBuf::from(&path),
                        content: content
                    };

                    filesMap.insert(name, tmpfile);

                },
                None => {
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
        }

        // Default parser
        match serde_json::from_str(&query) {
            Ok(req) => Ok((req,Some(filesMap))),
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

}

#[rocket::async_trait]
impl<'r, S> FromData<'r> for GraphQLUploadWrapper<S>
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
                        Ok(result) => Success(GraphQLUploadWrapper{
                            operations: GraphQLUploadOperationsRequest {
                                gql_request: result,
                            },
                            files: None
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
                        Ok(result) => Success(GraphQLUploadWrapper{
                            operations: GraphQLUploadOperationsRequest {
                                gql_request: result,
                            },
                            files: None
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                })
                .await
            }
            ProcessorType::MULTIPART => {
                Box::pin(async move {
                    match Self::from_multipart_body(data, content_type).await {
                        Ok(result) => Success(GraphQLUploadWrapper{
                            operations: GraphQLUploadOperationsRequest {
                                gql_request: result.0,
                            },
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
