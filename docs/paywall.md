# Paywall

## Principles

The paywall acts like a bridge beetween a payment network - Lightning network - and the 
access rights to a file or a content. 

The bridge can be implemented on different levels. 


## Globally protected API

The most basic paywall mechanism is to protect a whole API behind a proof of access. 

It is not implemented on this project.

## Route protected API

In this case the client must provide a token - the payment request - to access the endpoint of the API. 

The client uses the headers to provide the token. 

if no token or an invalid one is provided, the client should be redirected to an HTTP 402 response, providing an invoice - the payment request - to the client.

When the payment request is provided by the client, the server reads the invoice settlement state to provide access to the content. 

## GraphQL query protected API

On certain queries of a graphQL the client must provide a payment_request value to access the response of the API.

The client calls the query which will provide the invoice - the payment request - to the client the the graphQL errors property. 

When the payment request is provided as query's input, the server matches resource payment registry and reads the invoice settlement matching it. 

if no token is provided, or an invalid one,
the client will be provided an invoice - the payment request - 


## GraphQL field protected API