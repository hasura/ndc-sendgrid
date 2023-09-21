use std::collections::BTreeMap;
use indexmap::IndexMap;
use ndc_sdk::{
    connector::QueryError,
    models::{Argument, QueryRequest, QueryResponse, RowFieldValue, RowSet},
};

use super::configuration;
use super::schema::LIST_TEMPLATES_FUNCTION_NAME;
use super::sendgrid_api::{invoke_list_function_templates, ListTransactionalTemplatesParams};



// String::from("generations"),
// ArgumentInfo {
//     description: Some(String::from("Comma-delimited list specifying which generations of templates to return. Options are legacy, dynamic or legacy,dynamic")),
//     argument_type: nullable(named("String"))
// }),
// (
// String::from("page_size"),
// ArgumentInfo {
//     description: Some(String::from("The number of templates to be returned in each page of results")),
//     argument_type: named("Int")
// }),
// (
// String::from("page_token"),
// ArgumentInfo {
//     description: Some(String::from("A token corresponding to a specific page of results, as provided by metadata")),
//     argument_type: nullable(named("String"))
// }),


fn parse_list_templates_params(
    in_args: BTreeMap<String, Argument>,
) -> Result<ListTransactionalTemplatesParams, QueryError> {
    let page_size = in_args
        .get("page_size")
        .ok_or(QueryError::InvalidRequest(String::from(
            "Couldn't find 'page_size' field in arguments",
        )))?;

    let default_generations = Argument::Literal { value: serde_json::json!("legacy,dynamic") };
    let generations = in_args
        .get("generations")
        .unwrap_or(&default_generations);

    let page_token = in_args
        .get("page_token")
        .unwrap_or(&Argument::Literal { value: serde_json::Value::Null });

    match (generations, page_size, page_token) {
        ( Argument::Literal { value: generations_value },
          Argument::Literal { value: page_size_value },
          Argument::Literal { value: page_token_value } ) => {
            let generations_parsed = serde_json::from_value(generations_value.clone()).map_err(|err| {
                    QueryError::InvalidRequest(format!("Unable to deserialize 'params': {err}"))
                })?;
            let page_size_parsed = serde_json::from_value(page_size_value.clone()).map_err(|err| {
                    QueryError::InvalidRequest(format!("Unable to deserialize 'params': {err}"))
                })?;
            let page_token_parsed = serde_json::from_value(page_token_value.clone()).map_err(|err| {
                    QueryError::InvalidRequest(format!("Unable to deserialize 'params': {err}"))
                })?;
            Ok(
                ListTransactionalTemplatesParams {
                    generations: generations_parsed,
                    page_size: page_size_parsed,
                    page_token: page_token_parsed
                }
            )
          }
        _ => Err(QueryError::UnsupportedOperation(String::from(
            "Variables not currently supported",
        ))),
    }
}

pub async fn execute(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    query_request: QueryRequest,
) -> Result<QueryResponse, QueryError> {
    match query_request.collection.as_str() {
        LIST_TEMPLATES_FUNCTION_NAME => {
            let args = query_request.arguments;
            let params = parse_list_templates_params(args)?;
            let response =
                invoke_list_function_templates(http_client, &configuration.sendgrid_api_key, &params).await;

            match response {
                Ok(list_response) => {
                    let result = serde_json::to_value(list_response.result)
                        .map_err(|err| QueryError::Other(Box::new(err)))?;
                    let response_row =
                        IndexMap::from([(String::from("__value"), RowFieldValue(result))]);
                    Ok(QueryResponse(vec![RowSet {
                        rows: Some(vec![response_row]),
                        aggregates: None,
                    }]))
                }
                Err(err) => Err(QueryError::Other(Box::new(err))),
            }
        }
        unknown_collection => Err(QueryError::InvalidRequest(format!(
            "Unknown collection: {unknown_collection}"
        ))),
    }
}
