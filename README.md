# GraphQLN Api

 ![graphQLN](https://github.com/Asone/graphQLN/actions/workflows/rust.yml/badge.svg)

GraphQLN is a proof-of-concept of a [graphQL](https://graphql.org/) API with a built-in bitcoin [lightning network](https://en.wikipedia.org/wiki/Lightning_Network) paywall mechanism, built with [Rustlang](https://www.rust-lang.org/).

## Status

The project is still under development and lacks tests.
## Features

- User authentication protected mutations
- API paywall over Lightning
- Data query paywall over lightning 
## Documentation

An extended documentation is provided in the `docs` folder to help you understand how to install, configure and run the server : 

- [Installation](./docs/installation.md)
- [Configuration](./docs/configuration.md)
- [Paywall](./docs/paywall.md)
## Main dependencies

The project reliess on many dependencies to build and distribute the API. 
In order to understand how it is built and works, you can check the documentations of those dependencies : 

- [Rocket](https://rocket.rs/) : Provides the web server ([documentation](https://api.rocket.rs/v0.5-rc/rocket/))
- [Juniper](https://github.com/graphql-rust/juniper) : Provides the GraphQL engine ([documentation](https://docs.rs/juniper/0.15.7/juniper/))
- [Diesel](https://diesel.rs/) : Provides the ORM engine ([documentation](https://docs.diesel.rs/master/diesel/index.html))
- [tonic_lnd](https://github.com/Kixunil/tonic_lnd) : Provides the lightning network client based on the gRPC proto ([documentation](https://docs.rs/tonic_lnd/0.1.1/tonic_lnd/))
- [lightning-invoice]() : Provides utilities that allows to deserialize an invoice from a payment request ([documentation]())

