use std::collections::BTreeMap;

use ndc_sdk::{
    connector::QueryError,
    models::{Argument, QueryRequest, QueryResponse},
};

use super::configuration;
use super::schema::{ListTemplateRequest, TEMPLATES_FUNCTION_NAME};
use super::sendgrid_api::{invoke_get_function_template, GetFunctionTemplatesRequest};

fn parse_get_templates_arguments(
    in_args: BTreeMap<String, Argument>,
) -> Result<GetFunctionTemplatesRequest, QueryError> {
    let request = in_args
        .get("request")
        .ok_or(QueryError::InvalidRequest(String::from(
            "Couldn't find `request` field in arguments",
        )))?;
    match request {
        Argument::Literal { value } => {
            let req: ListTemplateRequest =
                serde_json::from_value(value.clone()).map_err(|err| {
                    QueryError::InvalidRequest(format!("Unable to deserialize 'request': {err}"))
                })?;
            let response = GetFunctionTemplatesRequest {
                page_size: req.page_size,
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
        TEMPLATES_FUNCTION_NAME => {
            let args = query_request.arguments;
            let sg_args = parse_get_templates_arguments(args)?;
            let response =
                invoke_get_function_template(&configuration.sendgrid_api_key, &sg_args).await;
            println!("{:#?}", response);
            Ok(QueryResponse(vec![]))
        }
        unknown_collection => Err(QueryError::InvalidRequest(format!(
            "Unknown collection: {unknown_collection}"
        ))),
    }
}
