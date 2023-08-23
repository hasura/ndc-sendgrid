use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq)]
pub struct GetFunctionTemplatesRequest {
    pub page_size: u32,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq)]
pub struct GetFunctionTemplatesResponse {
    // TODO
}

const SENDGRID_BASE_URL: &str = "https://api.sendgrid.com";

pub async fn invoke_get_function_template(
    SendGridApiKey(api_key): &SendGridApiKey,
    request: &GetFunctionTemplatesRequest,
) -> GetFunctionTemplatesResponse {

    let client = reqwest::Client::new();
    client
        .post(format!("{SENDGRID_BASE_URL}/v3/templates"))
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request)
        .send()
        .await
        .expect("Couldn't fetch")
        .json::<GetFunctionTemplatesResponse>()
        .await
        .expect("Couldn't deserialize")
}
