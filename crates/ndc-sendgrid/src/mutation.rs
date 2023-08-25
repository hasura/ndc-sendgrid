use ndc_sdk::{
    connector::MutationError,
    models::{Argument, MutationRequest, MutationResponse},
};
use std::collections::BTreeMap;

use super::configuration;
use super::schema;
use super::sendgrid_api;

fn parse_send_mail_args(
    in_args: BTreeMap<String, Argument>,
) -> Result<sendgrid_api::SendMailRequest, MutationError> {
    let args_request =
        in_args
            .get("request")
            .ok_or(MutationError::InvalidRequest(String::from(
                "Couldn't find 'request' field in arguments",
            )))?;
    match args_request {
        Argument::Literal { value } => {
            let schema_request = serde_json::from_value::<schema::SendMailRequest>(value.clone())
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
        Argument::Variable { .. } => Err(MutationError::UnsupportedOperation(String::from(
            "Variables not currently supported",
        ))),
    }
}

pub async fn execute(
    http_client: &reqwest::Client,
    configuration: &configuration::SendGridConfiguration,
    mutation_request: MutationRequest,
) -> Result<MutationResponse, MutationError> {
    todo!();
}
