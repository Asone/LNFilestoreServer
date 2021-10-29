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

## Available requests 

### Mutation

There is currently a single mutation available through the API, that allows you to create a post. Note that there is no restriction (like user guard) to create posts. 

Request : 

````
mutation {
  createPost(post: {
    title: "Ad lorem ipsum",
    excerpt: "This is a short description of the post",
    content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed laoreet suscipit ullamcorper. Etiam sit amet justo dapibus, elementum magna sit amet, faucibus risus. Nullam at augue in quam tristique posuere. Nullam congue dignissim odio non sagittis. Sed in libero erat. Maecenas dictum blandit purus. Suspendisse eget sem suscipit, auctor risus in, ornare orci. Curabitur id facilisis nisl, vitae interdum libero. Aenean commodo nulla sit amet arcu consectetur, non tristique purus elementum. Sed ex sem, blandit eleifend fringilla ac, sagittis auctor ipsum.",
    published: true,
    price: 100
  }){
    title
    excerpt
    price
  }
}
````

will return: 

````
{
  "data": {
    "createPost": {
      "title": "Ad lorem ipsum",
      "excerpt": "This is a short description of the post",
      "price": 100
    }
  }
}
````
### Queries

#### **Get posts list**

Request : 
```
{
  getPostsList{
    uuid
    title
    excerpt
    price
  }
}
```

will return something like : 

```
{
  "data": {
    "getPostsList": [
      {
        "uuid": "9f3711b4-f733-4911-9863-0c4ee575ca10",
        "title": "ad lorem ipsum",
        "excerpt": "alea jacta est",
        "price": 100
      },
      {
        "uuid": "e07677d1-4a45-422e-ac9b-a3a39cd91f0c",
        "title": "Ad lorem ipsum",
        "excerpt": "This is a short description of the post",
        "price": 100
      }
    ]
  }
}
```
#### **Get a single post**

This is the query where the paywall and most of the LN Network interaction is applied. 

You'll find the code block that handles the paywall [here](https://github.com/Asone/graphqln/blob/master/src/graphql/query.rs#L40)

The request takes an object with two fields : 
- The post uuid
- The payment request that should allow the server to identify the access to the content has been paid. This field is optional, and if not provided, the api will respond with an error providing an invoice.



## Main dependencies

The project reliess on many dependencies to build and distribute the API. 
In order to understand how it is built and works, you can check the documentations of those dependencies : 

- [Rocket](https://rocket.rs/) : Provides the web server ([documentation](https://api.rocket.rs/v0.5-rc/rocket/))
- [Juniper](https://github.com/graphql-rust/juniper) : Provides the GraphQL engine ([documentation](https://docs.rs/juniper/0.15.7/juniper/))
- [Diesel](https://diesel.rs/) : Provides the ORM engine ([documentation](https://docs.diesel.rs/master/diesel/index.html))
- [tonic_lnd](https://github.com/Kixunil/tonic_lnd) : Provides the lightning network client ([documentation](https://docs.rs/tonic_lnd/0.1.1/tonic_lnd/))
- [Documentation for generated LN objects retrieved through gRPC proto]()

