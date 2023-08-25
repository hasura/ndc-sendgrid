mod configuration;
mod connector;
mod schema;
mod mutation;
mod query;
mod sendgrid_api;

use connector::SendGridConnector;
use ndc_sdk::default_main::default_main;

#[tokio::main]
pub async fn main() {
    default_main::<SendGridConnector>().await.unwrap()
}
