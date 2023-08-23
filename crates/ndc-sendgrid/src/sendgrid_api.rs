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
