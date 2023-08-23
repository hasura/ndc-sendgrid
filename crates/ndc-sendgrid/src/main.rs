mod configuration;
mod connector;

use connector::SendGridConnector;
use ndc_sdk::default_main::default_main;

#[tokio::main]
pub async fn main() {
    default_main::<SendGridConnector>().await.unwrap()
}
