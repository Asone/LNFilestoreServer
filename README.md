# LND Filestore 
 
LN Filestore is a webserver to distribute files over the lightning network.

Its primary use is made for [umbrel](https://www.umbrel.com)
## Requirements 

In order to run the webserver you will have to provide access to a [postgresql](https://www.postgresql.org/) database and a [LND](https://github.com/lightningnetwork/lnd) synced node. 

See [configuration documentation](./docs/configuration.md) for more details.

If you want to run it on a docker-compose you can find an example in my [umbrel-apps](https://github.com/Asone/umbrel-apps/blob/master/lnfilestore/docker-compose.yml) fork. 


## Run

- x86 processors
````shell
docker run akbarworld/lnfilestoreapi

````
- arm64 processors
````
docker run akbarworld/lnfilestoreapi:umbrel
````

## Documentation

An extended documentation is provided in the `docs` folder to help you understand how to configure, build and run the server : 

- [Configuration](./docs/configuration.md)
- [Build](./docs/installation.md)

## Main dependencies

The project reliess on many dependencies to build and distribute the API. 
In order to understand how it is built and works, you can check the documentations of those dependencies : 

- [Rocket](https://rocket.rs/) : Provides the web server ([documentation](https://api.rocket.rs/v0.5-rc/rocket/))
- [Juniper](https://github.com/graphql-rust/juniper) : Provides the GraphQL engine ([documentation](https://docs.rs/juniper/0.15.7/juniper/))
- [Diesel](https://diesel.rs/) : Provides the ORM engine ([documentation](https://docs.diesel.rs/master/diesel/index.html))
- [tonic_lnd](https://github.com/Kixunil/tonic_lnd) : Provides the lightning network client based on the gRPC proto ([documentation](https://docs.rs/tonic_lnd/0.1.1/tonic_lnd/))
- [lightning-invoice](https://github.com/lightningdevkit/rust-lightning/) : Provides utilities that allows to deserialize an invoice from a payment request ([documentation](https://docs.rs/lightning-invoice/0.19.0/lightning_invoice/))

## Licence 
MIT Licence. 

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.