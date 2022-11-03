# Configuration

You can configure the server at runtime providing 
the following environment variables.

Some variables will be necessary to run the server.

Note that you can use the `.env.dist` template file.

## Database

LN Filestore Server requires access to a running Postgresql service. 

**Database user**
> DB_USER=user

**Database URL**
> DATABASE_URL=postgres://rustuser:rustuser@0.0.0.0:5433/test

**Database password**
> DB_PASSWORD=userpass

**Database migrations**

> DATABASE_RUN_MIGRATIONS_ON_IGNITE=false

Indicates if the migrations should be run on server launch. 
This is useful to build the database schema on first launch. 

Note that the further launches of the server won't overwrite the already ran migrations.

**Database seed**

> DATABASE_SEED_ON_IGNITE=true

Indicates if the database should be provided with seed data. 

This will seed the database with a default admin user which information can be provided through the `DEFAULT_ADMIN_*` parameters

## Default user information

**Default admin name**

> DEFAULT_ADMIN_NAME=Satoshi

**Default admin email**

> DATABASE_SEED_EMAIL=satoshi@nakamoto.btc

**Default admin password**

> DATABASE_SEED_PWD=craigwrightisnotsatoshi

## LND 

**LND address**

Note that the provided address **must** be an `https` url.

> LND_ADDRESS=https://0.0.0.0:10009

**LND Certfile path**
> LND_CERTFILE_PATH="src/lnd/config/lnd.cert"

**LND Macaroon path**

> LND_MACAROON_PATH=src/lnd/config/invoices.macaroon

You **must** provide a macaroon with read/write access on `invoices`. **Do not** provide admin macaroon for security reasons. 

If you're unfamiliar with macaroons you can find documentation about what are macaroons [here](https://github.com/lightningnetwork/lnd/blob/master/docs/macaroons.md).

## Rocket
Rocket handles environment configuration with prefixed `ROCKET_*` env values. 

You can  read the [official documentation](https://rocket.rs/v0.5-rc/guide/configuration/#environment-variables) for more information.

**Temporary directory**
>ROCKET_TEMP_DIR=tmp

The temporary directory for files and data

**SSL Configuration**
> ROCKET_TLS={certs="ssl/localhost.pem",key="ssl/localhost-key.pem"}

You can provide an SSL configuration in order to protect the server behind `https`.

See [official documentation]() for more information.
**Secret key**
>ROCKET_SECRET_KEY="hPRYyVRiMyxpw5sBB1XeCMN1kFsDCqKvBi2QJxBVHQk="

See [official documentation](https://api.rocket.rs/master/rocket/config/struct.SecretKey.html) for more information.
## Invoices

**Default invoice value**
>DEFAULT_INVOICE_VALUE=250

The default price - in satoshis - for an invoice.

**Default memo**
>DEFAULT_INVOICE_MEMO="Toto"

The default message to use when generating an invoice.

**Default lifetime**
>DEFAULT_INVOICE_EXPIRY=300

The default expiry time for an invoice

## Cookies

**secure cookie policy**

> COOKIES_IS_SECURE_POLICY=true

See [official cookie's rocket documentation](https://api.rocket.rs/v0.4/rocket/http/struct.Cookie.html#method.secure) for more details. 

See [general documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#security) for information around cookies secure flag.

**same site cookie policy**

> COOKIES_SAME_SITE_POLICY=strict

Possible values : `strict`, `none` or `lax`. 

Default policy is `lax`.

# JWT

**Token duration**
> JWT_TOKEN_DURATION=1000

Value represents seconds.

**Token secret**

> JWT_TOKEN_SECRET="secret"


## CORS  

**Origin policy**
> CORS_ORIGIN_POLICY="https://localhost:3000"

**Method policy**
> CORS_METHOD_POLICY="POST, GET, PATCH, OPTIONS"

**Headers policy**
> CORS_HEADERS_POLICY="Content-Type, Access-Control-Allow-Headers, Authorization, X-Requested-With"

**Credentials policy**
> CORS_CREDENTIALS_POLICY="true"

## Dev tools 

You can enable a [graphiQL]() instance for development : 
> ENABLE_DEV_TOOLS=true


### Configure diesel options

You need to create a `diesel.toml` file in the root folder to specify to diesel its configuration. You can use the `diesel.toml.dist` as a simple example of the configuration file.
