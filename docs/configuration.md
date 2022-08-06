## Set-up the project

In order to be able to launch the project you need to set-up a few configurations. Once all set-up, you shall be able to launch the server with

```
cargo run
```

You'll find then a [graphiQL](https://github.com/graphql/graphiql) interface on [http://localhost:8000](http://localhost:8000)
### Configure Database connection

The current project uses [postgres]() as database engine. 
To set-up the connection copy the `rocket.toml.dist` file as `rocket.toml` and fill the connection URL as mentioned. 

```
main_db = { url = "postgres://<user>:<password>@<host>/<db_name>"}
```

### Configure diesel options

You need to create a `diesel.toml` file in the root folder to specify to diesel its configuration. You can use the `diesel.toml.dist` as a simple example of the configuration file.

### Configure LND connection 

The current project uses LND server to handle Lightning network.

In a `.env` file in the root folder of the project, You'll need to provide :

- `LND_ADDRESS` : The address to reach the LND server
- `LND_CERTFILE_PATH` : the ssl certification file of your LND server
-  `LND_MACAROON_PATH` : The macaroon that will allow the rocket server to connect to your LND server. 

**Note that the current project requires a macaroon with at least invoice write/read access.**

You can use the `.env.dist` file as a template for that.

```
LND_ADDRESS="https://umbrel.local:10009"
LND_CERTFILE_PATH="path/to/the/lnd.cert"
LND_MACAROON_PATH="path/to/the/invoice.macaroon"
```
