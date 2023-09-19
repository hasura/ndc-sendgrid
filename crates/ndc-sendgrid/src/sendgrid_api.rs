use std::{collections::BTreeMap, fmt::Display};

use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SendGridApiKey(String);

pub enum ApiKeyError {
    CannotBeBlank,
}

impl SendGridApiKey {
    pub fn new(api_key: &str) -> Result<Self, ApiKeyError> {
        if api_key.is_empty() {
            Err(ApiKeyError::CannotBeBlank)
        } else {
            Ok(SendGridApiKey(String::from(api_key)))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListTransactionalTemplatesParams {
    pub generations: Option<String>,
    pub page_size: u32,
    pub page_token: Option<String>,
}

impl ListTransactionalTemplatesParams {
    pub fn to_query_params(self: &Self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        self.generations
            .as_ref()
            .map(|gen| params.push((String::from("generations"), gen.clone())));
        params.push((String::from("page_size"), self.page_size.to_string()));
        self.page_token
            .as_ref()
            .map(|page_token| params.push((String::from("page_token"), page_token.clone())));
        params
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ListTransactionalTemplatesResponse {
    pub result: Vec<TransactionalTemplate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionalTemplate {
    pub id: String,
    pub name: String,
    pub generation: String,
    pub updated_at: String,
    pub versions: Vec<TransactionTemplateVersion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionTemplateVersion {
    pub id: String,
    pub template_id: String,
    pub active: u32,
    pub name: String,
    pub subject: Option<String>,
    pub updated_at: String,
    pub generate_plain_content: bool,
    pub editor: String,
    pub thumbnail_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub errors: Option<Vec<ErrorItem>>,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Errors: ")?;
        match &self.errors {
            Some(errs) => {
                write!(f, "[")?;
                for (index, err) in errs.iter().enumerate() {
                    write!(f, "{}", err)?;
                    if index < errs.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            None => {
                write!(f, "[]")
            }
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorItem {
    pub message: String,
    pub error_id: String,
}

impl Display for ErrorItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_id, self.message)
    }
}

const SENDGRID_BASE_URL: &str = "https://api.sendgrid.com";

#[derive(Clone, Debug, Error)]
pub enum RequestError<Err> {
    #[error("API error: {error}")]
    ApiError { error: Err },
    #[error("Unexpected status code: {status_code}")]
    UnexpectedStatusCode { status_code: StatusCode },
    #[error("Unknown error: {error}")]
    OtherError { error: String },
}

#[derive(Serialize, Clone, Debug)]
pub struct SendMailRequest {
    pub personalizations: Vec<MailPersonalization>,
    pub from: MailAddress,
    pub reply_to_list: Vec<MailAddress>,
    pub subject: String,
    pub content: Vec<MailContent>,
    pub attachments: Option<Vec<MailAttachment>>,
    pub template_id: Option<String>,
    pub headers: Option<BTreeMap<String, String>>,
    pub send_at: Option<u32>,
    pub batch_id: Option<String>,
    pub asm: Option<UnsubscriptionSettings>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MailPersonalization {
    pub from: Option<MailAddress>,
    pub to: Vec<MailAddress>,
    pub cc: Option<Vec<MailAddress>>,
    pub bcc: Option<Vec<MailAddress>>,
    pub subject: Option<String>,
    pub headers: Option<BTreeMap<String, String>>,
    pub substitutions: Option<BTreeMap<String, String>>,
    pub dynamic_template_data: Option<BTreeMap<String, String>>,
    pub send_at: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MailAddress {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MailContent {
    pub r#type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MailAttachment {
    pub content: String,
    pub r#type: Option<String>,
    pub filename: String,
    pub disposition: Option<String>,
    pub content_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnsubscriptionSettings {
    pub group_id: String,
    pub groups_to_display: Option<Vec<String>>,
}

pub async fn invoke_list_function_templates(
    http_client: &reqwest::Client,
    SendGridApiKey(api_key): &SendGridApiKey,
    params: &ListTransactionalTemplatesParams,
) -> Result<ListTransactionalTemplatesResponse, RequestError<ErrorResponse>> {
    let response = http_client
        .get(format!("{SENDGRID_BASE_URL}/v3/templates"))
        .query(&(params.to_query_params()))
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await
        .map_err(|err| RequestError::OtherError {
            error: err.to_string(),
        })?;

    match response.status() {
        StatusCode::OK => response
            .json::<ListTransactionalTemplatesResponse>()
            .await
            .map_err(|err| RequestError::OtherError {
                error: err.to_string(),
            }),
        StatusCode::BAD_REQUEST => {
            let result = response.json::<ErrorResponse>().await;
            match result {
                Ok(err) => Err(RequestError::ApiError { error: err }),
                Err(other) => Err(RequestError::OtherError {
                    error: other.to_string(),
                }),
            }
        }
        other_code => Err(RequestError::UnexpectedStatusCode {
            status_code: other_code,
        }),
    }
}

pub async fn invoke_send_mail(
    http_client: &reqwest::Client,
    SendGridApiKey(api_key): &SendGridApiKey,
    request: &SendMailRequest,
) -> Result<(), RequestError<ErrorResponse>> {
    let response = http_client
        .post(format!("{SENDGRID_BASE_URL}/v3/mail/send"))
        .json(request)
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await
        .map_err(|err| RequestError::OtherError {
            error: err.to_string(),
        })?;

    match response.status() {
        StatusCode::ACCEPTED => Ok(()),
        StatusCode::BAD_REQUEST => {
            let result = response.json::<ErrorResponse>().await;
            match result {
                Ok(err) => Err(RequestError::ApiError { error: err }),
                Err(other) => Err(RequestError::OtherError {
                    error: other.to_string(),
                }),
            }
        }
        other_code => Err(RequestError::UnexpectedStatusCode {
            status_code: other_code,
        }),
    }
}
