use std::path::Path;
use async_trait::async_trait;
use ndc_sdk::connector;
use ndc_sdk::json_response::JsonResponse;
use ndc_sdk::models;

use super::configuration;
use super::mutation;
use super::query;
use super::schema;

#[derive(Clone, Default)]
pub struct SendGridConnector {}

#[derive(Clone, Debug)]
pub struct SendGridConnectorState {
    http_client: reqwest::Client
}

#[async_trait]
impl connector::Connector for SendGridConnector {
    /// The type of validated configuration
    type Configuration = configuration::SendGridConfiguration;
    /// The type of unserializable state
    type State = SendGridConnectorState;

    /// Validate the raw configuration provided by the user,
    /// returning a configuration error or a validated [`Connector::Configuration`].
    async fn parse_configuration(
        configuration_dir: impl AsRef<Path> + Send,
    ) -> Result<configuration::SendGridConfiguration, connector::ValidateError> {
        configuration::parse_configuration(configuration_dir)
    }

    /// Initialize the connector's in-memory state.
    ///
    /// For example, any connection pools, prepared queries,
    /// or other managed resources would be allocated here.
    ///
    /// In addition, this function should register any
    /// connector-specific metrics with the metrics registry.
    async fn try_init_state(
        _configuration: &configuration::SendGridConfiguration,
        _metrics: &mut prometheus::Registry,
    ) -> Result<SendGridConnectorState, connector::InitializationError> {
        Ok(SendGridConnectorState { http_client:reqwest::Client::new() })
    }

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
    ) -> Result<(), connector::FetchMetricsError> {
        Ok(())
    }

    /// Check the health of the connector.
    ///
    /// For example, this function should check that the connector
    /// is able to reach its data source over the network.
    async fn health_check(
        _configuration: &configuration::SendGridConfiguration,
        _state: &SendGridConnectorState,
    ) -> Result<(), connector::HealthError> {
        Ok(())
    }

    /// Get the connector's capabilities.
    ///
    /// This function implements the [capabilities endpoint](https://hasura.github.io/ndc-spec/specification/capabilities.html)
    /// from the NDC specification.
    async fn get_capabilities() -> JsonResponse<models::CapabilitiesResponse> {
        JsonResponse::Value(
            models::CapabilitiesResponse {
                version: String::from("0.1.0"),
                capabilities: models::Capabilities {
                    query: models::QueryCapabilities {
                        aggregates: None,
                        variables: None,
                        explain: None,
                    },
                    relationships: None,
                    mutation: models::MutationCapabilities {
                        transactional: None,
                        explain: None
                    },
                },
            }
        )
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        _configuation: &configuration::SendGridConfiguration,
    ) -> Result<JsonResponse<models::SchemaResponse>, connector::SchemaError> {
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
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        Err(connector::ExplainError::UnsupportedOperation(String::from("query explain is not supported")))
    }

    /// Explain a mutation by creating an execution plan
    ///
    /// This function implements the [mutation/explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn mutation_explain(
        _configuration: &Self::Configuration,
        _state: &Self::State,
        _request: models::MutationRequest,
    ) -> Result<JsonResponse<models::ExplainResponse>, connector::ExplainError> {
        Err(connector::ExplainError::UnsupportedOperation(String::from("mutation explain is not supported")))
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        request: models::MutationRequest,
    ) -> Result<JsonResponse<models::MutationResponse>, connector::MutationError> {
        mutation::execute(&state.http_client, configuration, request).await.map(JsonResponse::Value)
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        query_request: models::QueryRequest,
    ) -> Result<JsonResponse<models::QueryResponse>, connector::QueryError> {
        query::execute(&state.http_client, configuration, query_request).await.map(JsonResponse::Value)
    }
}
