use async_trait::async_trait;
use http::status::StatusCode;
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;
use std::path::Path;

use super::configuration;
use super::mutation;
use super::query;
use super::schema;

#[derive(Clone, Default)]
pub struct SendGridConnector {}

#[derive(Clone, Debug)]
pub struct SendGridConnectorState {
    http_client: reqwest::Client,
}

#[async_trait]
impl connector::ConnectorSetup for SendGridConnector {
    type Connector = SendGridConnector;

    /// Validate the configuration provided by the user, returning a configuration error or a
    /// validated [`Configuration`].
    ///
    /// The [`ParseError`] type is provided as a convenience to connector authors, to be used on
    /// error.
    async fn parse_configuration(
        &self,
        configuration_dir: impl AsRef<Path> + Send,
    ) -> connector::Result<<Self::Connector as connector::Connector>::Configuration> {
        configuration::parse_configuration(configuration_dir)
    }

    /// Initialize the connector's in-memory state.
    ///
    /// For example, any connection pools, prepared queries, or other managed resources would be
    /// allocated here.
    ///
    /// In addition, this function should register any connector-specific metrics with the metrics
    /// registry.
    ///
    /// This may be called repeatedly until it succeeds.
    async fn try_init_state(
        &self,
        _configuration: &<Self::Connector as connector::Connector>::Configuration,
        _metrics: &mut prometheus::Registry,
    ) -> connector::Result<<Self::Connector as connector::Connector>::State> {
        Ok(SendGridConnectorState {
            http_client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl connector::Connector for SendGridConnector {
    /// The type of validated configuration
    type Configuration = configuration::SendGridConfiguration;
    /// The type of unserializable state
    type State = SendGridConnectorState;

    /// Update any metrics from the state
    ///
    /// Note: some metrics can be updated directly, and do not
    /// need to be updated here. This function can be useful to
    /// query metrics which cannot be updated directly, e.g.
    /// the number of idle connections in a connection pool
    /// can be polled but not updated directly.
    fn fetch_metrics(
        _configuration: &configuration::SendGridConfiguration,
        _state: &SendGridConnectorState,
    ) -> connector::Result<()> {
        Ok(())
    }

    /// Check the health of the connector.
    ///
    /// This should simply verify that the connector is ready to start accepting
    /// requests. It should not verify that external data sources are available.
    ///
    /// For most use cases, the default implementation should be fine.
    async fn get_health_readiness(
        _configuration: &configuration::SendGridConfiguration,
        _state: &SendGridConnectorState,
    ) -> connector::Result<()> {
        Ok(())
    }

    /// Get the connector's capabilities.
    ///
    /// This function implements the [capabilities endpoint](https://hasura.github.io/ndc-spec/specification/capabilities.html)
    /// from the NDC specification.
    async fn get_capabilities() -> models::Capabilities {
        models::Capabilities {
            query: models::QueryCapabilities {
                aggregates: None,
                variables: None,
                explain: None,
                exists: models::ExistsCapabilities {
                    nested_collections: None,
                },
                nested_fields: models::NestedFieldCapabilities {
                    filter_by: None,
                    order_by: None,
                    aggregates: None,
                },
            },
            relationships: None,
            mutation: models::MutationCapabilities {
                transactional: None,
                explain: None,
            },
        }
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        _configuation: &configuration::SendGridConfiguration,
    ) -> connector::Result<JsonResponse<models::SchemaResponse>> {
        Ok(JsonResponse::Value(schema::make_schema_response()))
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [query/explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn query_explain(
        _configuration: &configuration::SendGridConfiguration,
        _state: &SendGridConnectorState,
        _query_request: models::QueryRequest,
    ) -> connector::Result<JsonResponse<models::ExplainResponse>> {
        Err(
            connector::ErrorResponse::from("query explain is not supported".to_owned())
                .with_status_code(StatusCode::NOT_IMPLEMENTED),
        )
    }

    /// Explain a mutation by creating an execution plan
    ///
    /// This function implements the [mutation/explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn mutation_explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _request: models::MutationRequest,
    ) -> connector::Result<JsonResponse<models::ExplainResponse>> {
        Err(
            connector::ErrorResponse::from("mutation explain is not supported".to_owned())
                .with_status_code(StatusCode::NOT_IMPLEMENTED),
        )
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        request: models::MutationRequest,
    ) -> connector::Result<JsonResponse<models::MutationResponse>> {
        mutation::execute(&state.http_client, configuration, request)
            .await
            .map(JsonResponse::Value)
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        query_request: models::QueryRequest,
    ) -> connector::Result<JsonResponse<models::QueryResponse>> {
        query::execute(&state.http_client, configuration, query_request)
            .await
            .map(JsonResponse::Value)
    }
}
