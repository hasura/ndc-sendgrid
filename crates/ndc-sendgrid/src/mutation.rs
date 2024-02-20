use ndc_sdk::connector::MutationError;
use ndc_sdk::models::{
    MutationOperation, MutationOperationResults, MutationRequest, MutationResponse, NestedField
};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::fields::eval_nested_field;
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
    }
}

async fn process_send_mail(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    arguments: BTreeMap<String, Value>,
    fields: Option<NestedField>,
) -> Result<MutationOperationResults, MutationError> {
    let request = parse_send_mail_args(&arguments)?;

    sendgrid_api::invoke_send_mail(http_client, &configuration.sendgrid_api_key, &request)
        .await
        .map_err(|err| MutationError::Other(Box::new(err)))?;

    let batch_id_value = request
        .batch_id
        .clone()
        .map_or(Value::Null, |id| Value::String(id));
    let result_value = Value::Object(serde_json::Map::from_iter([("batch_id".to_string(), batch_id_value)]));

    let projected_result_value = match fields {
        Some(fields) => eval_nested_field(result_value, &fields)?.0,
        None => result_value
    };

    Ok(MutationOperationResults::Procedure { result: projected_result_value })
}

fn parse_send_mail_args(
    in_args: &BTreeMap<String, Value>,
) -> Result<sendgrid_api::SendMailRequest, MutationError> {
    let args_request = serde_json::Value::Object(serde_json::Map::from_iter(in_args.clone()));
    let schema_request = serde_json::from_value::<schema::SendMailRequest>(args_request)
        .map_err(|err| {
            MutationError::InvalidRequest(format!("Unable to deserialize arguments: {err}"))
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
