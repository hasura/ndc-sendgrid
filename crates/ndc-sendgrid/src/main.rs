mod configuration;
mod connector;
mod sendgrid_api;

use connector::SendGridConnector;
use ndc_sdk::default_main::default_main;

#[tokio::main]
pub async fn main() {
    default_main::<SendGridConnector>().await.unwrap()
}
