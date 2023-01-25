#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let report_tokens = &["your report tokens"];
    let secondary_client_id = "your secondary client id";
    let response = client
        .credit_relay_create(report_tokens, secondary_client_id)
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}