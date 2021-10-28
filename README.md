# GraphQLN Api

This project is a demo of a [graphQL](https://graphql.org/) API with a built-in bitcoin [lightning network](https://en.wikipedia.org/wiki/Lightning_Network) paywall mechanism, built with [Rustlang](https://www.rust-lang.org/).


## Install project

1. Clone the project :

```
git clone <project_url>
```

2. go to the folder and install dependencies : 
```
cargo install --path .
```

## Set-up the project

In order to be able to launch the project you need to set-up a few configurations. 

### Database connection

The current project uses [postgres]() as database engine. 
To set-up the connection copy the `rocket.toml.dist` file as `rocket.toml` and fill the connection URL as mentioned. 

```
main_db = { url = "postgres://<user>:<password>@<host>/<db_name>"}
```

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


