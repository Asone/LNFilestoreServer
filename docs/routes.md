# HTTP and GraphQL requests

## Routes

You will find the definition of the different routes in the [routes](../src/routes/) folder.

### / 

The root path will load a [graphiQL](https://github.com/graphql/graphiql) instance 
which you can use for development and test cases.

However do note that we intend to later provide a mechanism that will allow to disable the instance
based on environment variables as we might not want to keep an acess opened on production environments.

### POST /auth

The `/auth` route provides the request to authenticate onto the server. 

You will have to provide a `POST` http request with the following form data keys and values : 

The username intended to log in :
> username

The password field : 
> password

If authentication is successful the server will provide an empty response with `HTTP/200` and a `session` cookie. 

### GET /file/:uuid?invoice=:invoice

The `/file/:uuid` route is used to retrieve files and data protected through LN payment. 

You will have to provide and `uuid` in order to specify to the server the registered file you want to retrieve. 

the `invoice` parameter is optional to make the request but mandatory to retrieve the file. 

If no invoice is provided the server will reply with an `HTTP/402`response which body will contain a json with a `payment_request` field that represents the `invoice` value to be paid in order to provide the file. 

Once paid, the same value will be used as the `invoice` parameter in the request to prove the file/data can be accessed. 

### POST /graphql

Provides the GraphQL API. See below

## GraphQL Schema

An export of the graphql Schema is provided [here](./resources/schema.gql). You can import this schema into GraphiQL and Altaïr to get the full documentation of the GraphQL API


