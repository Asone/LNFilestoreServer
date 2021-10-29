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

````graphql
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

````json
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
```graphql
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

```json
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


For example, providing the request like this : 

```graphql
{
  getPost(post:{
    uuid: "9f3711b4-f733-4911-9863-0c4ee575ca10"
  }){
    uuid
    title
    excerpt
    content
    price
  }
}
```

You'll get a response similar to this : 

```json
{
  "data": null,
  "errors": [
    {
      "message": "Payable post. Payment not found.. Use provided payment request.",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "getPost"
      ],
      "extensions": {
        "payment_request": "lnbc1u1pshhszcpp5e3wpuwldl92zumajqs58k69stru6g9rc43nw0v7uy5rnk8vl7f0sdpagaexzurg29xyugzp2pyjqur0wd6zqcn40ysr5grpvssxcmmjv4kjq6tswd6k6cqzpgxqyz5vqsp5ggd3dps9r27dcmxtmj463uct653n2agqttmjhm3qw6wgkfzaqw9s9qyyssq8zga2evqh8lt7kv40269puz3xehezxqvauhz4he0zvyke0x642q33jy85za4qtwa5p24x0vh5ve2p5ztqw64mlpsuwj5ml3ke8rl67spzzhwhv",
        "r_hash": "cc5c1e3bedf9542e6fb204287b68b058f9a41478ac66e7b3dc25073b1d9ff25f"
      }
    }
  ]
}
```

Note the [extensions](https://dgraph.io/docs/graphql/api/requests/#extensions-field) sub-object which are part of the graphQL spec.
It provides a `payment_request` which will allow user to pay to access the content.

The user can try to refetch the content providing the same `payment_request` within the request.

```graphql
{
  getPost(post:{
    uuid: "9f3711b4-f733-4911-9863-0c4ee575ca10",
    paymentRequest: "lnbc1u1pshhszcpp5e3wpuwldl92zumajqs58k69stru6g9rc43nw0v7uy5rnk8vl7f0sdpagaexzurg29xyugzp2pyjqur0wd6zqcn40ysr5grpvssxcmmjv4kjq6tswd6k6cqzpgxqyz5vqsp5ggd3dps9r27dcmxtmj463uct653n2agqttmjhm3qw6wgkfzaqw9s9qyyssq8zga2evqh8lt7kv40269puz3xehezxqvauhz4he0zvyke0x642q33jy85za4qtwa5p24x0vh5ve2p5ztqw64mlpsuwj5ml3ke8rl67spzzhwhv"
  }){
    uuid
    title
    excerpt
    content
    price
  }
}
```

When provided, the server will check the invoice state for the provided `payment_request` and its local association with the requested post to ensure the user can access the content. 

A few cases can happen. 

If user requests with another payment request that is not the one provided, it will get a similar response to this : 

```json
{
  "data": null,
  "errors": [
    {
      "message": "No recorded payment request related to the requested post found with the payment requested provided. Proceed to payment with the provided request payment",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "getPost"
      ],
      "extensions": {
        "payment_request": "lnbc1u1pshh3vypp5zyzf88glqv7gkuvqn95j97nzlt32xk8c9tu8t8dzywyax0vdw72sdpagaexzurg29xyugzp2pyjqur0wd6zqcn40ysr5grpvssxcmmjv4kjq6tswd6k6cqzpgxqyz5vqsp5275n2cjqjgly9trkmucfux3krxw9z5na7wjmtklvua0j8tsw0pts9qyyssqw5kcm6r0zp4d5uu0p9zq5ehx9zen4svt63tvj20pa5kwfevv3p7x863f7mz4spa4w6p326jkegjq3gwtf8jzzr72nukyn8aw2s2gayqppmrhmq",
        "r_hash": "1104939d1f033c8b7180996922fa62fae2a358f82af8759da22389d33d8d7795"
      }
    }
  ]
}
```

Note how the server provides automatically a new invoice so the front-end can provide it to the user. 

If user requests the data without paying but providing the payment request he will get something similar to this : 

```json
{
  "data": null,
  "errors": [
    {
      "message": "Awaiting for payment to be done.",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "getPost"
      ]
    }
  ]
}
```

If invoice expired, user will get something similar to this : 


```json
{
  "data": null,
  "errors": [
    {
      "message": "Payment expired or canceled. Proceed to payment with the provided request payment",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "getPost"
      ],
      "extensions": {
        "payment_request": "lnbc1u1pshh3vypp5zyzf88glqv7gkuvqn95j97nzlt32xk8c9tu8t8dzywyax0vdw72sdpagaexzurg29xyugzp2pyjqur0wd6zqcn40ysr5grpvssxcmmjv4kjq6tswd6k6cqzpgxqyz5vqsp5275n2cjqjgly9trkmucfux3krxw9z5na7wjmtklvua0j8tsw0pts9qyyssqw5kcm6r0zp4d5uu0p9zq5ehx9zen4svt63tvj20pa5kwfevv3p7x863f7mz4spa4w6p326jkegjq3gwtf8jzzr72nukyn8aw2s2gayqppmrhmq",
        "r_hash": "1104939d1f033c8b7180996922fa62fae2a358f82af8759da22389d33d8d7795"
      }
    }
  ]
}
```

Note how the server regenerates automatically a new invoice to be provided to the user. 

Finally, once the user has paid the invoice, he will get the content with a response similar to this : 

```json
{
  "data": {
    "getPost": {
      "uuid": "9f3711b4-f733-4911-9863-0c4ee575ca10",
      "title": "ad lorem ipsum",
      "excerpt": "alea jacta est",
      "content": "ad lorem ipsum dolor sit amet fluctuat nec mergitur rosa rosae rosam",
      "price": 100
    }
  }
}
```

Note that the user can reuse the payment request as many times as he wants as 
we do store the association invoice - content and server will check to the LND instance that the invoice is settled. 



## Main dependencies

The project reliess on many dependencies to build and distribute the API. 
In order to understand how it is built and works, you can check the documentations of those dependencies : 

- [Rocket](https://rocket.rs/) : Provides the web server ([documentation](https://api.rocket.rs/v0.5-rc/rocket/))
- [Juniper](https://github.com/graphql-rust/juniper) : Provides the GraphQL engine ([documentation](https://docs.rs/juniper/0.15.7/juniper/))
- [Diesel](https://diesel.rs/) : Provides the ORM engine ([documentation](https://docs.diesel.rs/master/diesel/index.html))
- [tonic_lnd](https://github.com/Kixunil/tonic_lnd) : Provides the lightning network client based on the gRPC proto ([documentation](https://docs.rs/tonic_lnd/0.1.1/tonic_lnd/))
- [lightning-invoice]() : Provides utilities that allows to deserialize an invoice from a payment request ([documentation]())

