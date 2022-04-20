use multer::{bytes::Bytes, Multipart};
use rocket::{
    data::{self, ByteUnit, FromData, ToByteUnit},
    form::Error,
    http::{ContentType, Status},
    outcome::Outcome::{Failure, Forward, Success},
    route::Outcome,
    Data, Request,
};
use std::sync::Arc;
use std::{collections::HashMap, env, path::PathBuf};

use juniper::{
    http::{self, GraphQLBatchRequest},
    DefaultScalarValue, ScalarValue,
};

use crate::graphql_upload_operations_request::GraphQLOperationsRequest;

use crate::temp_file::TempFile;

// This shall be deferable to env
const BODY_LIMIT: u64 = 1024 * 100;

#[derive(Debug)]
pub enum MultipartFormParsingError {
    BoundaryParsingError,
}

enum ProcessorType {
    JSON,
    GRAPHQL,
    MULTIPART,
    UKNOWN,
}

/// A Wrapper that handles HTTP GraphQL requests for [`Rocket`](https://rocket.rs/)
/// with multipart/form-data support that follows the Apollo's
/// [`unofficial specification`](https://github.com/jaydenseric/graphql-multipart-request-spec.).
///
/// # Main concept
///
/// The current struct is nothing more than a wrapper
/// that will return two fields :
/// - `operations` : Contains the graphQL Request to be executed.
///     This object is nothing more than a replica
///     of the original [`GraphQLRequest`](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/src/lib.rs#L64)
///     object.
/// - `files` : An optional HashMap that contains the buffered files data.
///
/// Note that the wrapper also replicates original juniper_rocket [`GraphQLRequest`]
/// parsing behavior for `application/json` and `application/graphql` requests.
///   
/// ## How to use
///
/// You can load the GraphQLUploadWrapper the same way
/// you load the GraphQLRequest as both are data guards.
/// The main difference will be that instead, you'll call the
/// execution of the query through the `operations` property
/// of the wrapper.
///
///  Below is basic example :
///
/// ```
/// #[rocket::post("/upload", data = "<request>")]
/// pub async fn upload<'r>(
///    request: GraphQLUploadWrapper,
///    schema: &State<Schema>,
/// ) -> GraphQLResponse {
///   request.operations.execute(&*schema, &Context).await
/// }
/// ```
///
/// ## Fetching the uploaded files
///
/// In order to fetch the uploaded files
/// You'll need to implement your own context object
/// That will pass the buffered files to your execution methods.
///
/// Example :
/// ```
/// struct Ctx{
///   files: Option<HashMap<String, TempFile>>
/// };
/// impl juniper::Context for Ctx {}
/// ```
///
/// You'll then be able to inject the buffered files to your
/// operations like this :
/// ```
/// struct Ctx{ files: Option<HashMap<String, TempFile>> };
/// impl juniper::Context for Ctx {}
///
/// #[rocket::post("/upload", data = "<request>")]
/// pub async fn upload<'r>(
///    request: GraphQLUploadWrapper,
///    schema: &State<Schema>,
/// ) -> GraphQLResponse {
///   request.operations.execute(&*schema, &Ctx{ files: request.files }).await
/// }
/// ```
///
/// ## Notes about processed files
///
/// The Wrapper does nothing special with the uploaded files aside
/// allocating them in heap memory through the Hashmap which means
/// they won't be stored anywhere, not even in a temporary folder,
/// unless you decide to.
///
/// See [`TempFile`] for more available data and information around uploaded files.
#[derive(Debug, PartialEq)]
pub struct GraphQLUploadWrapper<S = DefaultScalarValue>
where
    S: ScalarValue,
{
    pub operations: GraphQLOperationsRequest<S>,
    pub files: Option<HashMap<String, TempFile>>,
}

impl<S> GraphQLUploadWrapper<S>
where
    S: ScalarValue,
{
    // Retrieves files
    pub async fn get_files(&self) -> &Option<HashMap<String, TempFile>> {
        &self.files
    }

    //   Body reader for application/json content type.
    //   This method replicates the original handler from juniper_rocket
    async fn from_json_body<'r>(data: Data<'r>) -> Result<GraphQLBatchRequest<S>, Outcome<'r>> {
        use rocket::tokio::io::AsyncReadExt as _;

        let mut reader = data.open(BODY_LIMIT.bytes());
        let mut body = String::new();
        let reader_result = reader.read_to_string(&mut body).await;
        match reader_result {
            Ok(_) => match serde_json::from_str(&body) {
                Ok(req) => Ok(req),
                Err(e) => Err(Failure(Status::BadRequest)),
            },
            Err(e) => Err(Failure(Status::BadRequest)),
        }
    }

    //   Body reader for application/graphql content type.
    //   This method replicates the original handler from juniper_rocket
    async fn from_graphql_body<'r>(data: Data<'r>) -> Result<GraphQLBatchRequest<S>, Outcome<'r>> {
        use rocket::tokio::io::AsyncReadExt as _;

        let mut reader = data.open(BODY_LIMIT.bytes());
        let mut body = String::new();
        let reader_result = reader.read_to_string(&mut body).await;
        match reader_result {
            Ok(_) => Ok(GraphQLBatchRequest::Single(http::GraphQLRequest::new(
                body, None, None,
            ))),
            Err(e) => Err(Failure(Status::BadRequest)),
        }
    }

    /// Body reader for multipart/form-data content type.
    async fn from_multipart_body<'r>(
        data: Data<'r>,
        content_type: &ContentType,
        file_limit: ByteUnit,
    ) -> Result<(GraphQLBatchRequest<S>, Option<HashMap<String, TempFile>>), Error<'r>> {
        // Builds a void query for development
        let mut query: String = String::new();
        let mut map: String = String::new();
        let boundary = Self::get_boundary(content_type).unwrap();

        // Create and read a datastream from the request body
        let reader = data.open(file_limit);
        let stream = tokio_util::io::ReaderStream::new(reader);

        // Create a multipart object based on multer
        let mut multipart = Multipart::new(stream, boundary);

        let mut files_map: HashMap<String, TempFile> = HashMap::new();

        // Iterate on the form fields, which can be
        // either text content or binary.
        while let Some(entry) = multipart.next_field().await.unwrap() {
            let field_name = match entry.name() {
                Some(name) => Arc::<&str>::from(name).to_string(),
                None => continue,
            };

            let name = field_name;

            let path = format!("{}", env::temp_dir().display());

            match entry.content_type().as_ref() {
                Some(_) => {
                    let file_name = match entry.file_name() {
                        Some(filename) => Arc::<&str>::from(filename).to_string(),
                        None => continue,
                    };

                    let content = match entry.bytes().await {
                        Ok(d) => d,
                        Err(_) => Bytes::new(),
                    };

                    let mut tmpfile = TempFile {
                        name: file_name,
                        size: Some(content.len()),
                        local_path: PathBuf::from(&path),
                        content: content,
                    };

                    files_map.insert(name, tmpfile);
                }
                None => {
                    let content_data = entry.text().await;

                    // If field name is operations which should be the graphQL Query
                    // according to spec
                    match content_data {
                        Ok(result) => {
                            if name.as_str() == "operations" {
                                query = result;
                                continue;
                            }

                            // if name.as_str() == "map" {
                            //     map = result;
                            // }
                        }
                        Err(_) => continue,
                    };
                }
            }
        }

        // Default parser
        match serde_json::from_str(&query) {
            Ok(req) => Ok((req, Some(files_map))),
            Err(_) => Err(rocket::form::Error::validation(
                "The provided request could not be parsed.",
            )),
        }
    }

    ///  Returns an enum value for a specific processors based on request Content-type
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

    /// Extracts the boundary for a multipart/form-data request
    pub fn get_boundary(content_type: &ContentType) -> Result<&str, MultipartFormParsingError> {
        match content_type.params().find(|&(k, _)| k == "boundary") {
            Some(s) => Ok(s.1),
            None => Err(MultipartFormParsingError::BoundaryParsingError),
        }
    }
}

struct Config {}

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
                    match Self::from_json_body(data).await {
                        Ok(result) => Success(GraphQLUploadWrapper {
                            operations: GraphQLOperationsRequest(result),
                            files: None,
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                    // Success(Self {})
                })
                .await
            }
            ProcessorType::GRAPHQL => {
                Box::pin(async move {
                    match Self::from_graphql_body(data).await {
                        Ok(result) => Success(GraphQLUploadWrapper {
                            operations: GraphQLOperationsRequest(result),
                            files: None,
                        }),
                        Err(error) => Failure((Status::BadRequest, format!("{}", error))),
                    }
                })
                .await
            }
            ProcessorType::MULTIPART => {
                Box::pin(async move {
                    match Self::from_multipart_body(
                        data,
                        content_type,
                        req.limits().get("data-form").unwrap(),
                    )
                    .await
                    {
                        Ok(result) => Success(GraphQLUploadWrapper {
                            operations: GraphQLOperationsRequest(result.0),
                            files: result.1,
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
