#[cfg(test)]
mod tests {

    use crate::app_build;
    use rocket::http::ContentType;

    // Sets environment variables for testing as .env file is not loaded
    fn set_test_env_variables() -> () {
        std::env::set_var("LND_ADDRESS", "https://umbrel.local:10009");
        std::env::set_var("LND_CERTFILE_PATH", "src/lnd/config/lnd.cert");
        std::env::set_var("LND_MACAROON_PATH", "src/lnd/config/admin.macaroon");
    }

    // Tests rocket ignition
    #[rocket::async_test]
    async fn test_rocket() {
        use rocket::local::asynchronous::Client;
        let _client = Client::tracked(app_build()).await.unwrap();
    }

    // This test ensures graphql gets provided through the expected endpoint
    #[ignore]
    #[rocket::async_test]
    async fn test_graphql_http_endpoint() {
        use rocket::local::asynchronous::Client;

        set_test_env_variables();

        let client = Client::tracked(app_build()).await.unwrap();

        let request = client
            .post(uri!(crate::app::post_graphql_handler))
            .header(ContentType::JSON)
            .body(r#"{"query":"query IntrospectionQuery {__schema {queryType { name }mutationType { name } subscriptionType { name }}}","operationName":"IntrospectionQuery"}"#);
        let response = request.dispatch().await;

        let content = response.into_string().await;

        assert!(content.is_some());
    }

    // async fn test_graphql_
}
