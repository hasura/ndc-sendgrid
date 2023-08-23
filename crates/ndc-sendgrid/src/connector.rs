use async_trait::async_trait;
use ndc_sdk::connector;
use ndc_sdk::models;

use super::configuration;

#[derive(Clone, Default)]
pub struct SendGridConnector {}

#[async_trait]
impl connector::Connector for SendGridConnector {
    /// RawConfiguration is what the user specifies as JSON
    type RawConfiguration = configuration::RawSendGridConfiguration;
    /// The type of validated configuration
    type Configuration = configuration::SendGridConfiguration;
    /// The type of unserializable state
    type State = ();

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
        _configuration: &Self::Configuration,
        _metrics: &mut prometheus::Registry,
    ) -> Result<Self::State, connector::InitializationError> {
        todo!()
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
        _state: &Self::State,
    ) -> Result<(), connector::FetchMetricsError> {
        todo!()
    }

    /// Check the health of the connector.
    ///
    /// For example, this function should check that the connector
    /// is able to reach its data source over the network.
    async fn health_check(
        _configuration: &Self::Configuration,
        _state: &Self::State,
    ) -> Result<(), connector::HealthError> {
        Ok(())
    }

    /// Get the connector's capabilities.
    ///
    /// This function implements the [capabilities endpoint](https://hasura.github.io/ndc-spec/specification/capabilities.html)
    /// from the NDC specification.
    async fn get_capabilities() -> models::CapabilitiesResponse {
        todo!()
    }

    /// Get the connector's schema.
    ///
    /// This function implements the [schema endpoint](https://hasura.github.io/ndc-spec/specification/schema/index.html)
    /// from the NDC specification.
    async fn get_schema(
        _configuation: &configuration::SendGridConfiguration,
    ) -> Result<models::SchemaResponse, connector::SchemaError> {
        todo!()
    }

    /// Explain a query by creating an execution plan
    ///
    /// This function implements the [explain endpoint](https://hasura.github.io/ndc-spec/specification/explain.html)
    /// from the NDC specification.
    async fn explain(
        _configuration: &configuration::SendGridConfiguration,
        _state: &Self::State,
        _query_request: models::QueryRequest,
    ) -> Result<models::ExplainResponse, connector::ExplainError> {
        todo!()
    }

    /// Execute a mutation
    ///
    /// This function implements the [mutation endpoint](https://hasura.github.io/ndc-spec/specification/mutations/index.html)
    /// from the NDC specification.
    async fn mutation(
        _configuration: &configuration::SendGridConfiguration,
        _state: &Self::State,
        _request: models::MutationRequest,
    ) -> Result<models::MutationResponse, connector::MutationError> {
        todo!("mutations are currently not implemented")
    }

    /// Execute a query
    ///
    /// This function implements the [query endpoint](https://hasura.github.io/ndc-spec/specification/queries/index.html)
    /// from the NDC specification.
    async fn query(
        _configuration: &configuration::SendGridConfiguration,
        _state: &Self::State,
        _query_request: models::QueryRequest,
    ) -> Result<models::QueryResponse, connector::QueryError> {
        todo!()
    }
}
