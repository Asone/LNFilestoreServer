use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

/// Represents a Cors Config object
/// that will be used to build Cors Policy on
/// Server runtime
struct CorsConfig {
    allow_origin: String,
    allow_methods: String,
    allow_headers: String,
    allow_credentials: String,
}

///
/// Allows us to modify CORS default parameters
///
pub struct Cors;

#[rocket::async_trait]
///
/// Implements the Cors Policy
///
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    // Fairing provides the cors policy on request response
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // Gets the config
        let config = Cors::get_config();

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            config.allow_origin,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            config.allow_methods,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            config.allow_headers,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Credentials",
            config.allow_credentials,
        ));
    }
}

impl Cors {
    // Creates a config with environment variables if set and default values for
    // values not set in env variables
    fn get_config() -> CorsConfig {
        let origin_policy = env::var("CORS_ORIGIN_POLICY").unwrap_or("*".to_string());
        let allow_methods =
            env::var("CORS_METHOD_POLICY").unwrap_or("POST, GET, PATCH, OPTIONS".to_string());
        let allow_headers = env::var("CORS_HEADERS_POLICY").unwrap_or("*".to_string());
        let allow_credentials = env::var("CORS_CREDENTIALS_POLICY").unwrap_or("false".to_string());

        return CorsConfig {
            allow_origin: origin_policy.to_owned(),
            allow_methods: allow_methods.to_owned(),
            allow_headers: allow_headers.to_owned(),
            allow_credentials: allow_credentials.to_owned(),
        };
    }
}
