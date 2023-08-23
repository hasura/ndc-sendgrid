use std::collections::{HashMap, BTreeMap};

use ndc_sdk::{models::{self, Argument, QueryResponse}, connector::{self, QueryError}};

use crate::{configuration, sendgrid_api::{GetFunctionTemplatesRequest, invoke_get_function_template}};
use super::schema::TEMPLATES_FUNCTION_NAME;

fn parse_get_templates_arguments(in_args: BTreeMap<String, Argument>) -> Result<GetFunctionTemplatesRequest, QueryError> {
    let request = in_args.get("request").ok_or(QueryError::InvalidRequest(String::from("Couldn't find `request` field in arguments")))?;
    match request {
        Argument::Literal { value } => {
            let request_object = value.as_object().ok_or(QueryError::InvalidRequest(String::from("`request` argument should be an object")))?;
            let page_size_value = request_object.get("page_size").ok_or(QueryError::InvalidRequest(String::from("`request` argument should contain `page_size` field")))?;
            let page_size_int = page_size_value.as_u64().ok_or(QueryError::InvalidRequest(String::from("`page_size` field should be a positive integer")))?;
            let page_size_u32 = u32::try_from(page_size_int).map_err(|err| QueryError::InvalidRequest(String::from(format!("`page_size` {page_size_int} is too big"))))?;
            let response = GetFunctionTemplatesRequest {
                page_size: page_size_u32
            };
            Ok(response)
        }
        Argument::Variable { name } => {
            Err(QueryError::InvalidRequest(String::from("doesn't currently support variables")))
        }
    }
}

pub async fn execute(
    configuration: &configuration::SendGridConfiguration,
    query_request: models::QueryRequest,
) -> Result<models::QueryResponse, connector::QueryError> {
    match query_request.collection.as_str() {
        TEMPLATES_FUNCTION_NAME => {
            let args = query_request.arguments;
            let sg_args = parse_get_templates_arguments(args)?;
            let response = invoke_get_function_template(&configuration.sendgrid_api_key, &sg_args).await;
            println!("{:#?}", response);
            Ok(QueryResponse(vec!()))
        }
        x => {
            todo!("lol2")
        }
    }
}