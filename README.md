# SendGrid Connector

The SendGrid Native Data Connector allows for connecting to the SendGrid v3 API and exposing its functionality from your
Hasura API.

While this is a functional implementation of the SendGrid API, it also serves as a minimal example of an "Action"
style connector using the [Rust Data Connector SDK](https://github.com/hasura/ndc-hub#rusk-sdk).

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

If you wish to use this connector with your Hasura projects, the best instructions can be found on the
[Hasura Hub SendGrid Connector listing](https://hasura.io/connectors/sendgrid).

The following steps will allow you to deploy the connector and use it in a Hasura V3 project:

* Create a Hasura V3 Project (or use an existing project)
* Ensure that you have a metadata definition
* Create a configuration for the SendGrid Connector referencing your credentials:
     `sendgrid.connector.configuration.json`
     ```
     {"version": 1, "sendgrid_api_key": "YOUR-API-KEY-HERE" }
     ```
* Run the following command to deploy the connector
* Ensure you are logged in to Hasura CLI
     ```
     hasura3 cloud login --pat 'YOUR-HASURA-TOKEN'
     ```
* Deploy the connector
     ```
     hasura3 connector create sendgrid:v1 \
     --github-repo-url https://github.com/hasura/ndc-sendgrid/tree/main \
     --config-file ./sendgrid.connector.configuration.json
     ```
* Ensure that your deployed connector is referenced from your metadata with the service token
* Edit your metadata using the LSP support to import the defined schema, functions, procedures
* Deploy or update your Hasura cloud project
     ```
     hasura3 cloud build create --project-id my-project-id --metadata-file metadata.hml
     ```
* View in your cloud console, access via the graphql API


## Development

Check out information on how to contribute to the development of this connector [here](./docs/development.md).

