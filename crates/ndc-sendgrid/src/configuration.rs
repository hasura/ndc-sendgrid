use ndc_sdk::connector;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{env, path::Path};

use super::sendgrid_api::{ApiKeyError, SendGridApiKey};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SendGridConfiguration {
    pub sendgrid_api_key: SendGridApiKey,
}

pub fn parse_configuration(
    _configuration_dir: impl AsRef<Path> + Send,
) -> connector::Result<SendGridConfiguration> {
    match env::var("SENDGRID_API_KEY") {
        Ok(key) => SendGridApiKey::new(key.as_str())
            .map(|api_key| SendGridConfiguration {
                sendgrid_api_key: api_key,
            })
            .map_err(|err| match err {
                ApiKeyError::CannotBeBlank => connector::ErrorResponse::from(
                    "The SENDGRID_API_KEY environment variable cannot be blank".to_owned(),
                ),
            }),
        Err(env::VarError::NotPresent) => Err(connector::ErrorResponse::from(
            "The SENDGRID_API_KEY environment variable is required".to_owned(),
        )),
        Err(env::VarError::NotUnicode(_)) => Err(connector::ErrorResponse::from(
            "The SENDGRID_API_KEY environment variable value is not valid unicode".to_owned(),
        )),
    }
}
