# Configuration

You can configure the server at runtime providing 
the following environment variables.

Some variables will be necessary to run the server. 
Environment variables used at runtime are listed below :  

| Key   |      Default value      | Required | 
|----------|:--------------------:|---------:|
| DATABASE_URL| |yes|
| DB_PASSWORD|                    | yes      |
| DB_USER| | yes |
| DATABASE_RUN_MIGRATIONS_ON_IGNITE| true | no |
| LND_ADDRESS|  | yes |
| LND_CERTFILE_PATH|  | yes |
| LND_MACAROON_PATH|  | yes |
| PAYWALL_TXS_NAME_CONTENT| buy {} | no |
| QUERY_PAYWALL_TIMER|800 | no |
| QUERY_PAYWALL_DEFAULT_PRICE| 50 | no |
| API_PAYWALL_TIMER| 1000 | no |
| API_PAYWALL_PRICE| 1000 | no |
| DEFAULT_INVOICE_VALUE| 250 | no |
| DEFAULT_INVOICE_MEMO| Toto | no |
| DEFAULT_INVOICE_EXPIRY| 300 | no |
| MAX_FILE_SIZE| 30000 | no |
| ROCKET_TEMP_DIR| tmp | no |
| ROCKET_TLS| | no |
| ROCKET_SECRET_KEY| | yes | 
| COOKIES_IS_SECURE| false | no |
| JWT_TOKEN_DURATION|1000 |
| JWT_TOKEN_SECRET|secret | | yes |
| CORS_ORIGIN_POLICY| * | no |
| CORS_METHOD_POLICY|POST, GET, PATCH, OPTIONS | no |
| CORS_HEADERS_POLICY|Content-Type,  Access-Control-Allow-Headers, Authorization, X-Requested-With | no |
| CORS_CREDENTIALS_POLICY|true | no |


Note that you can use the `.env.dist` file as a template for that.

### Configure diesel options

You need to create a `diesel.toml` file in the root folder to specify to diesel its configuration. You can use the `diesel.toml.dist` as a simple example of the configuration file.

### Configure LND connection 

The current project uses LND server to handle Lightning network.

You'll need to provide :

- `LND_ADDRESS` : The address to reach the LND server
- `LND_CERTFILE_PATH` : the ssl certification file of your LND server
-  `LND_MACAROON_PATH` : The macaroon that will allow the rocket server to connect to your LND server. 

**Note that the current project requires a macaroon with at least invoice write/read access.**

```
LND_ADDRESS="https://umbrel.local:10009"
LND_CERTFILE_PATH="path/to/the/lnd.cert"
LND_MACAROON_PATH="path/to/the/invoice.macaroon"
```
