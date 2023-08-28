use indexmap::IndexMap;
use ndc_sdk::connector::MutationError;
use ndc_sdk::models::{
    Field, MutationOperation, MutationOperationResults, MutationRequest, MutationResponse,
    RowFieldValue,
};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::schema::SEND_MAIL;

use super::configuration;
use super::schema;
use super::sendgrid_api;

pub async fn execute(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    mutation_request: MutationRequest,
) -> Result<MutationResponse, MutationError> {
    let mut operation_results = vec![];

    for operation in mutation_request.operations {
        let result = process_operation(http_client, configuration, operation).await?;
        operation_results.push(result)
    }

    Ok(MutationResponse {
        operation_results: operation_results,
    })
}

async fn process_operation(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    mutation_operation: MutationOperation,
) -> Result<MutationOperationResults, MutationError> {
    match mutation_operation {
        MutationOperation::Procedure {
            name,
            arguments,
            fields,
        } => match name.as_str() {
            SEND_MAIL => process_send_mail(http_client, configuration, arguments, fields).await,
            unknown_procedure => Err(MutationError::InvalidRequest(format!(
                "Unknown procedure: {unknown_procedure}"
            ))),
        },
        MutationOperation::Insert { .. } => Err(MutationError::UnsupportedOperation(String::from(
            "Insert mutations currently not supported",
        ))),
        MutationOperation::Update { .. } => Err(MutationError::UnsupportedOperation(String::from(
            "Update mutations currently not supported",
        ))),
        MutationOperation::Delete { .. } => Err(MutationError::UnsupportedOperation(String::from(
            "Delete mutations currently not supported",
        ))),
    }
}

async fn process_send_mail(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    arguments: BTreeMap<String, Value>,
    fields: Option<IndexMap<String, Field>>,
) -> Result<MutationOperationResults, MutationError> {
    let request = parse_send_mail_args(&arguments)?;
    sendgrid_api::invoke_send_mail(http_client, &configuration.sendgrid_api_key, &request)
        .await
        .map_err(|err| MutationError::Other(Box::new(err)))?;

    let mut row = IndexMap::new();

    if let Some(fields) = fields {
        for (field_name, field) in fields {
            match field {
                Field::Column { column } => {
                    let field_value = match column.as_str() {
                        "batch_id" => RowFieldValue(
                            request
                                .batch_id
                                .clone()
                                .map_or(Value::Null, |id| Value::String(id)),
                        ),
                        other_column => {
                            return Err(MutationError::InvalidRequest(format!(
                                "Unknown column {other_column}"
                            )))
                        }
                    };
                    row.insert(field_name, field_value);
                }
                Field::Relationship { relationship, .. } => {
                    return Err(MutationError::InvalidRequest(format!(
                        "Unexpected relationship {relationship} in field {field_name}"
                    )))
                }
            }
        }
    }

    Ok(MutationOperationResults {
        affected_rows: 1,
        returning: Some(vec![row]),
    })
}

fn parse_send_mail_args(
    in_args: &BTreeMap<String, Value>,
) -> Result<sendgrid_api::SendMailRequest, MutationError> {
    let args_request =
        in_args
            .get("request")
            .ok_or(MutationError::InvalidRequest(String::from(
                "Couldn't find 'request' field in arguments",
            )))?;

    let schema_request = serde_json::from_value::<schema::SendMailRequest>(args_request.clone())
        .map_err(|err| {
            MutationError::InvalidRequest(format!("Unable to deserialize 'request': {err}"))
        })?;

    let request = sendgrid_api::SendMailRequest {
        personalizations: schema_request
            .personalizations
            .into_iter()
            .map(|personalization| sendgrid_api::MailPersonalization {
                from: personalization.from,
                to: personalization.to,
                cc: personalization.cc,
                bcc: personalization.bcc,
                subject: personalization.subject,
                headers: personalization.headers.map(|headers| {
                    headers
                        .into_iter()
                        .map(|header| (header.name, header.value))
                        .collect()
                }),
                substitutions: personalization.substitutions.map(|substitutions| {
                    substitutions
                        .into_iter()
                        .map(|substitution| (substitution.tag, substitution.value))
                        .collect()
                }),
                dynamic_template_data: personalization.dynamic_template_data.map(|vars| {
                    vars.into_iter()
                        .map(|var| (var.variable, var.value))
                        .collect()
                }),
                send_at: personalization.send_at,
            })
            .collect(),
        from: schema_request.from,
        reply_to_list: schema_request.reply_to_list,
        subject: schema_request.subject,
        content: schema_request.content,
        attachments: schema_request.attachments,
        template_id: schema_request.template_id,
        headers: schema_request.headers.map(|headers| {
            headers
                .into_iter()
                .map(|header| (header.name, header.value))
                .collect()
        }),
        send_at: schema_request.send_at,
        batch_id: schema_request.batch_id,
        asm: schema_request.asm,
    };
    Ok(request)
}
