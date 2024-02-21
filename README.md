# SendGrid Connector

> [!WARNING]
> This connector has been updated to support the Hasura DDN Beta, and will not work with the Alpha.

The SendGrid Native Data Connector allows for connecting to the SendGrid v3 API and exposing its functionality from your Hasura API.
While this is a functional implementation of the SendGrid API, it also serves as a minimal example of an "Action" style connector using the [Rust Data Connector SDK](https://github.com/hasura/ndc-hub#rusk-sdk).

* [SendGrid Connector information in the Hasura Connectors directory](https://hasura.io/connectors/sendgrid)
* [Hasura V3 Documentation](https://hasura.io/docs/3.0)

In order to use this connector you will need to:

* Create a [SendGrid API account](https://signup.sendgrid.com/)
* Create an [API key](https://app.sendgrid.com/settings/api_keys)
* Log in to A Hasura CLI Session
* Create a Pre-Shared Token for service authentication between the Hasura V3 Engine and your connector

## Features

This connector is a minimal implementation of the SendGrid v3 API functions:

* Sending mail (the `send_mail` procedure)
* Getting a list of email templates (the `list_templates` function)

It also serves as an example of how an `Action` style connector can be implemented in Hasura V3.

## For Hasura Users
TBD

## For Developers

The following instructions are for developers who wish to contribute to the SendGrid Connector.

### Build

Prerequisites:

1. Install [rustup](https://www.rust-lang.org/tools/install).

Commands:

```
cargo build
SENDGRID_API_KEY="YOUR-API-KEY-HERE" cargo run -- serve --configuration .
```

### Docker

The `Dockerfile` is used by the `connector create` command and can be tested as follows:

```
docker build . --tag ndc-sendgrid
docker run -it -e SENDGRID_API_KEY="YOUR-API-KEY-HERE" ndc-sendgrid
```
