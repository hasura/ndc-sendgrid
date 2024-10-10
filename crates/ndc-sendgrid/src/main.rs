mod configuration;
mod connector;
mod fields;
mod mutation;
mod query;
mod schema;
mod sendgrid_api;

use connector::SendGridConnector;
use ndc_sdk::default_main::default_main;

#[tokio::main]
pub async fn main() {
    default_main::<SendGridConnector>().await.unwrap()
}
