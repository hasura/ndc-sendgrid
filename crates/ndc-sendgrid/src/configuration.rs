use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct RawSendGridConfiguration {}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct SendGridConfiguration {}
