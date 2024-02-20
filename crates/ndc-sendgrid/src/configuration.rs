use std::{env, path::Path};
use ndc_sdk::connector::{self, InvalidRange, KeyOrIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::sendgrid_api::{ApiKeyError, SendGridApiKey};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SendGridConfiguration {
    pub sendgrid_api_key: SendGridApiKey,
}

pub fn parse_configuration(
    _configuration_dir: impl AsRef<Path> + Send
) -> Result<SendGridConfiguration, connector::ValidateError> {

    match env::var("SENDGRID_API_KEY") {
        Ok(key) => SendGridApiKey::new(key.as_str())
            .map(|api_key| SendGridConfiguration {
                sendgrid_api_key: api_key,
            })
            .map_err(|err| match err {
                ApiKeyError::CannotBeBlank => {
                    mk_single_error("sendgrid_api_key", "sendgrid_api_key cannot be blank")
                }
            }),
        Err(env::VarError::NotPresent) => Err(mk_single_error(
            "SENDGRID_API_KEY",
            "The SENDGRID_API_KEY environment variable is required",
        )),
        Err(env::VarError::NotUnicode(_)) => Err(mk_single_error(
            "SENDGRID_API_KEY",
            "The SENDGRID_API_KEY environment variable value is not valid unicode",
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
