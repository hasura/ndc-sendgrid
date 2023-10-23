# [connectorName] Development

Instructions for developers who wish to contribute or build upon the connector:

### Build

Prerequisites:

1. Install [rustup](https://www.rust-lang.org/tools/install).

Commands:

```
cargo build
cargo run serve --configuration <(echo '{"version": 1, "sendgrid_api_key":"YOUR-API-KEY-HERE"}')
```

### Docker

The `Dockerfile` is used by the `connector create` command and can be tested as follows:

```
docker build . --tag ndc-sendgrid
docker run -it --v ./sendgrid.connector.configuration.json:/config.json ndc-sendgrid
```
