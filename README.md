# SendGrid Connector

The SendGrid Native Data Connector allows for connecting to the SendGrid v3 API and exposing its functionality from your Hasura API.
While this is a functional implementation of the SendGrid API, it also serves as a minimal example of an "Action" style connector using the [Rust Data Connector SDK](https://github.com/hasura/ndc-hub#rusk-sdk).

* [Hasura DDN Documentation](https://hasura.io/docs/3.0)

In order to use this connector you will need to:

* Create a [SendGrid API account](https://signup.sendgrid.com/)
* Create an [API key](https://app.sendgrid.com/settings/api_keys)
* A Hasura DDN project (see the [Getting Started guide](https://hasura.io/docs/3.0/getting-started/overview/))

## Features

This connector is a minimal implementation of the SendGrid v3 API functions:

* Sending mail (the `send_mail` procedure)
* Getting a list of email templates (the `list_templates` function)

It also serves as an example of how an `Action` style connector can be implemented in Hasura V3.

## For Hasura Users
Add the SendGrid connector to your DDN project by running

```
> ddn connector init -i
```

Select the SendGrid connector from the list and provide a name for the connector and your SendGrid API key.

Then you need to introspect the connector to get its schema:

```
> ddn connector introspect <connector name>
```

And then you can add all the SendGrid commands to your supergraph:

```
> ddn command add <connector name> "*"
```

You can now build your supergraph, run it locally, and open the Hasura Console to try it out:

```
> ddn supergraph build local
> ddn run docker-start
> ddn console --local
```

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
```
docker build . --tag ndc-sendgrid
docker run --rm -it -e SENDGRID_API_KEY="YOUR-API-KEY-HERE" -p 8080:8080 ndc-sendgrid
```

## Support & Troubleshooting

The documentation and community will help you troubleshoot most issues.
If you have encountered a bug or need to get in touch with us, you can contact us using one of the following channels:

- Support & feedback: [Discord](https://discord.gg/hasura)
- Issue & bug tracking: [GitHub issues](https://github.com/hasura/graphql-engine/issues)
- Follow product updates: [@HasuraHQ](https://twitter.com/hasurahq)
- Talk to us on our [website chat](https://hasura.io)
