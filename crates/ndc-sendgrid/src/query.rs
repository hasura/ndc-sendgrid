use std::collections::BTreeMap;

use indexmap::IndexMap;
use ndc_sdk::{
    connector::QueryError,
    models::{Argument, QueryRequest, QueryResponse, RowFieldValue, RowSet},
};

use super::configuration;
use super::schema::{ListTemplateRequest, LIST_TEMPLATES_FUNCTION_NAME};
use super::sendgrid_api::{invoke_get_function_template, ListTransactionalTemplatesParams};

fn parse_list_templates_params(
    in_args: BTreeMap<String, Argument>,
) -> Result<ListTransactionalTemplatesParams, QueryError> {
    let request = in_args
        .get("params")
        .ok_or(QueryError::InvalidRequest(String::from(
            "Couldn't find 'params' field in arguments",
        )))?;
    match request {
        Argument::Literal { value } => {
            let req: ListTemplateRequest =
                serde_json::from_value(value.clone()).map_err(|err| {
                    QueryError::InvalidRequest(format!("Unable to deserialize 'params': {err}"))
                })?;
            let response = ListTransactionalTemplatesParams {
                generations: req.generations,
                page_size: req.page_size,
                page_token: req.page_token,
            };
            Ok(response)
        }
        Argument::Variable { .. } => Err(QueryError::UnsupportedOperation(String::from(
            "Variables not currently supported",
        ))),
    }
}

pub async fn execute(
    configuration: &configuration::SendGridConfiguration,
    query_request: QueryRequest,
) -> Result<QueryResponse, QueryError> {
    match query_request.collection.as_str() {
        LIST_TEMPLATES_FUNCTION_NAME => {
            let args = query_request.arguments;
            let params = parse_list_templates_params(args)?;
            let response =
                invoke_get_function_template(&configuration.sendgrid_api_key, &params).await;

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
