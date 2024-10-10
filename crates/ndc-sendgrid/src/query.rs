use indexmap::IndexMap;
use ndc_sdk::{
    connector::{self, QueryError},
    models::{Argument, ArgumentName, QueryRequest, QueryResponse, RowSet},
};
use std::collections::BTreeMap;

use crate::fields::eval_row;

use super::configuration;
use super::schema::LIST_TEMPLATES_FUNCTION_NAME;
use super::sendgrid_api::{invoke_list_function_templates, ListTransactionalTemplatesParams};

fn parse_list_templates_params(
    in_args: BTreeMap<ArgumentName, Argument>,
) -> connector::Result<ListTransactionalTemplatesParams> {
    let page_size = in_args.get("page_size").ok_or_else(|| {
        QueryError::new_invalid_request(&"Couldn't find 'page_size' field in arguments")
    })?;

    let default_generations = Argument::Literal {
        value: serde_json::json!("legacy,dynamic"),
    };
    let generations = in_args.get("generations").unwrap_or(&default_generations);

    let page_token = in_args.get("page_token").unwrap_or(&Argument::Literal {
        value: serde_json::Value::Null,
    });

    match (generations, page_size, page_token) {
        (
            Argument::Literal {
                value: generations_value,
            },
            Argument::Literal {
                value: page_size_value,
            },
            Argument::Literal {
                value: page_token_value,
            },
        ) => {
            let generations_parsed =
                serde_json::from_value(generations_value.clone()).map_err(|err| {
                    QueryError::new_invalid_request(&format!(
                        "Unable to deserialize 'params': {err}"
                    ))
                })?;
            let page_size_parsed =
                serde_json::from_value(page_size_value.clone()).map_err(|err| {
                    QueryError::new_invalid_request(&format!(
                        "Unable to deserialize 'params': {err}"
                    ))
                })?;
            let page_token_parsed =
                serde_json::from_value(page_token_value.clone()).map_err(|err| {
                    QueryError::new_invalid_request(&format!(
                        "Unable to deserialize 'params': {err}"
                    ))
                })?;
            Ok(ListTransactionalTemplatesParams {
                generations: generations_parsed,
                page_size: page_size_parsed,
                page_token: page_token_parsed,
            })
        }
        _ => {
            Err(QueryError::new_unsupported_operation(&"Variables not currently supported").into())
        }
    }
}

pub async fn execute(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    query_request: QueryRequest,
) -> connector::Result<QueryResponse> {
    match query_request.collection.as_str() {
        LIST_TEMPLATES_FUNCTION_NAME => {
            let args = query_request.arguments;
            let params = parse_list_templates_params(args)?;
            let response = invoke_list_function_templates(
                http_client,
                &configuration.sendgrid_api_key,
                &params,
            )
            .await;

            match response {
                Ok(list_response) => {
                    let result = serde_json::to_value(list_response.result)
                        .map_err(|err| connector::ErrorResponse::from(err.to_string()))?;
                    let result_row = IndexMap::from([(String::from("__value"), result)]);
                    let projected_row = query_request
                        .query
                        .fields
                        .map(|fields| eval_row(&fields, &result_row))
                        .transpose()?;
                    Ok(QueryResponse(vec![RowSet {
                        rows: projected_row.map(|row| vec![row]),
                        aggregates: None,
                    }]))
                }
                Err(err) => Err(connector::ErrorResponse::from(err.to_string())),
            }
        }
        unknown_collection => Err(QueryError::new_invalid_request(&format!(
            "Unknown collection: {unknown_collection}"
        ))
        .into()),
    }
}
