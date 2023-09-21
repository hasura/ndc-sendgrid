use indexmap::IndexMap;
use ndc_sdk::connector::MutationError;
use ndc_sdk::models::{
    Field, MutationOperation, MutationOperationResults, MutationRequest, MutationResponse,
    RowFieldValue,
};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::schema::SEND_MAIL;
use crate::sendgrid_api::{SimpleSendMailRequest, MailAddress, MailContent, MailPersonalization};

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

fn complicate_request(simple_request: SimpleSendMailRequest) -> sendgrid_api::SendMailRequest {

    let personalization = MailPersonalization {
        from: Some(simple_request.from.clone()),
        to: vec!(simple_request.to),
        cc: simple_request.cc.map(|x| vec!(x)),
        bcc: simple_request.bcc.map(|x| vec!(x)),
        subject: Some(simple_request.subject.clone()),
        headers: None,
        substitutions: None,
        dynamic_template_data: None,
        send_at: simple_request.send_at
    };

    sendgrid_api::SendMailRequest {
        personalizations: vec!(personalization),
        from: simple_request.from,
        reply_to_list: vec!(),
        subject: simple_request.subject,
        content: vec!(simple_request.content),
        attachments: simple_request.attachment.map(|a| vec!(a)),
        template_id: simple_request.template_id,
        headers: None,
        send_at: simple_request.send_at,
        batch_id: simple_request.batch_id,
        asm: simple_request.asm
    }
}

async fn process_send_mail(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    arguments: BTreeMap<String, Value>,
    fields: Option<IndexMap<String, Field>>,
) -> Result<MutationOperationResults, MutationError> {
    let simple_request = parse_simple_send_mail_args(&arguments)?;
    let request = complicate_request(simple_request);

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

fn invalid_arg(err: &str) -> MutationError {
    MutationError::InvalidRequest(format!("Couldn't find '{err}' field in arguments"))
}

fn invalid_deserialize(arg: &str, err: serde_json::Error) -> MutationError {
    MutationError::InvalidRequest(format!("Unable to deserialize argument '{arg}': {err}"))
}

fn parse_simple_send_mail_args(
    in_args: &BTreeMap<String, Value>,
) -> Result<sendgrid_api::SimpleSendMailRequest, MutationError> {
    
    let args_to           = in_args.get("to").ok_or(invalid_arg("request"))?;
    let args_cc           = in_args.get("cc");
    let args_bcc          = in_args.get("bcc");
    let args_from         = in_args.get("from").ok_or(invalid_arg("from"))?;
    let args_reply_to     = in_args.get("reply_to");
    let args_subject      = in_args.get("subject").ok_or(invalid_arg("subject"))?;
    let args_content      = in_args.get("content").ok_or(invalid_arg("content"))?;
    let args_content_type = in_args.get("content_type").ok_or(invalid_arg("content_type"))?;
    let args_attachment   = in_args.get("attachment");
    let args_template_id  = in_args.get("template_id");
    let args_send_at      = in_args.get("send_at");
    let args_batch_id     = in_args.get("batch_id");
    let args_asm          = in_args.get("asm");

    let to           = serde_json::from_value(args_to.clone()).map_err(|err| invalid_deserialize("to", err))?;
    let cc           = args_cc.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("cc", err))).unwrap_or(Ok(None))?;
    let bcc          = args_bcc.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("bcc", err))).unwrap_or(Ok(None))?;
    let from         = serde_json::from_value(args_from.clone()).map_err(|err| invalid_deserialize("from", err))?;
    let reply_to     = args_reply_to.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("reply_to", err))).unwrap_or(Ok(None))?;
    let subject      = serde_json::from_value(args_subject.clone()).map_err(|err| invalid_deserialize("subject", err))?;
    let content      = serde_json::from_value(args_content.clone()).map_err(|err| invalid_deserialize("content", err))?;
    let content_type = serde_json::from_value(args_content_type.clone()).map_err(|err| invalid_deserialize("content", err))?;
    let attachment   = args_attachment.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("attachment", err))).unwrap_or(Ok(None))?;
    let template_id  = args_template_id.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("template_id", err))).unwrap_or(Ok(None))?;
    let send_at      = args_send_at.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("send_at", err))).unwrap_or(Ok(None))?;
    let batch_id     = args_batch_id.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("batch_id", err))).unwrap_or(Ok(None))?;
    let asm          = args_asm.map(|x| serde_json::from_value(x.clone()).map_err(|err| invalid_deserialize("asm", err))).unwrap_or(Ok(None))?;

    let request = sendgrid_api::SimpleSendMailRequest {
        to: MailAddress { email: to, name: None },
        cc: cc.map(|x| MailAddress { email: x, name: None }),
        bcc: bcc.map(|x| MailAddress { email: x, name: None }),
        from: MailAddress { email: from, name: None },
        reply_to: reply_to.map(|x| MailAddress { email: x, name: None }),
        subject,
        content: MailContent { r#type: content_type, value: content },
        attachment,
        template_id,
        send_at,
        batch_id,
        asm,
    };
    Ok(request)
}

fn _parse_send_mail_args(
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
