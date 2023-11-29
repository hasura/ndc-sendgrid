use ndc_sdk::connector::{self, InvalidRange, KeyOrIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::sendgrid_api::{ApiKeyError, SendGridApiKey};

const CURRENT_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct RawSendGridConfiguration {
    pub version: u32,
    pub sendgrid_api_key: Option<String>,
}

impl Default for RawSendGridConfiguration {
    fn default() -> RawSendGridConfiguration {
        RawSendGridConfiguration {
            version: CURRENT_VERSION,
            sendgrid_api_key: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SendGridConfiguration {
    pub sendgrid_api_key: SendGridApiKey,
}

pub fn validate_raw_configuration(
    raw_configuration: RawSendGridConfiguration,
) -> Result<SendGridConfiguration, connector::ValidateError> {
    if raw_configuration.version != CURRENT_VERSION {
        return Err(mk_single_error("version", "Unknown configuration version"));
    }

    match &raw_configuration.sendgrid_api_key {
        Some(key) => SendGridApiKey::new(key)
            .map(|api_key| SendGridConfiguration {
                sendgrid_api_key: api_key,
            })
            .map_err(|err| match err {
                ApiKeyError::CannotBeBlank => {
                    mk_single_error("sendgrid_api_key", "sendgrid_api_key cannot be blank")
                }
            }),
        None => Err(mk_single_error(
            "sendgrid_api_key",
            "sendgrid_api_key is required",
        )),
    }
}

fn mk_single_error(key: &str, message: &str) -> connector::ValidateError {
    let errs = vec![InvalidRange {
        path: vec![KeyOrIndex::Key(String::from(key))],
        message: String::from(message),
    }];
    connector::ValidateError::ValidateError(errs)
}
