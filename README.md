# SendGrid Native Data Connector
The SendGrid Native Data Connector allows for connecting to the SendGrid v3 API and exposing that from your Hasura API.

## Features
This connector is currently very much a work-in-progress and exposes the following limited functionality:
* Sending mail (the `send_mail` procedure)
* Getting a list of email templates (the `list_templates` function)

## Build

### Prerequisites
1. Create a [SendGrid API account](https://signup.sendgrid.com/) and [create an API key](https://app.sendgrid.com/settings/api_keys).
2. Install [rustup](https://www.rust-lang.org/tools/install).

### Compile
```
cargo build
```

### Run
```
cargo run serve --configuration <(echo '{"version": 1, "sendgrid_api_key":"YOUR-API-KEY-HERE"}')
```
