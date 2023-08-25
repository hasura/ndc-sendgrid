use async_trait::async_trait;
use ndc_sdk::connector;
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
    /// RawConfiguration is what the user specifies as JSON
    type RawConfiguration = configuration::RawSendGridConfiguration;
    /// The type of validated configuration
    type Configuration = configuration::SendGridConfiguration;
    /// The type of unserializable state
    type State = SendGridConnectorState;

    fn make_empty_configuration() -> configuration::RawSendGridConfiguration {
        configuration::RawSendGridConfiguration::default()
    }

    /// Configure a configuration maybe?
    async fn update_configuration(
        args: &configuration::RawSendGridConfiguration,
    ) -> Result<configuration::RawSendGridConfiguration, connector::UpdateConfigurationError> {
        Ok(args.clone())
    }

    /// Validate the raw configuration provided by the user,
    /// returning a configuration error or a validated [`Connector::Configuration`].
    async fn validate_raw_configuration(
        configuration: &configuration::RawSendGridConfiguration,
    ) -> Result<configuration::SendGridConfiguration, connector::ValidateError> {
        configuration::validate_raw_configuration(configuration)
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
    async fn get_capabilities() -> models::CapabilitiesResponse {
        models::CapabilitiesResponse {
            versions: String::from("^0.1.0"),
            capabilities: models::Capabilities {
                query: Some(models::QueryCapabilities {
                    relation_comparisons: None,
                    order_by_aggregate: None,
                    foreach: None,
                }),
                explain: None,
                relationships: None,
                mutations: Some(models::MutationCapabilities {
                    nested_inserts: None,
                    returning: None,
                }),
            },
        }
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        _configuation: &configuration::SendGridConfiguration,
    ) -> Result<models::SchemaResponse, connector::SchemaError> {
        Ok(schema::make_schema_response())
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn explain(
        _configuration: &configuration::SendGridConfiguration,
        _state: &SendGridConnectorState,
        _query_request: models::QueryRequest,
    ) -> Result<models::ExplainResponse, connector::ExplainError> {
        Err(connector::ExplainError::UnsupportedOperation(String::from("explain is not supported")))
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        request: models::MutationRequest,
    ) -> Result<models::MutationResponse, connector::MutationError> {
        mutation::execute(&state.http_client, configuration, request).await
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        configuration: &configuration::SendGridConfiguration,
        state: &SendGridConnectorState,
        query_request: models::QueryRequest,
    ) -> Result<models::QueryResponse, connector::QueryError> {
        query::execute(&state.http_client, configuration, query_request).await
    }
}
