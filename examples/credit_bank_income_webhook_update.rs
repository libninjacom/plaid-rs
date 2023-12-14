#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let enable_webhooks = true;
    let user_token = "your user token";
    let response = client
        .credit_bank_income_webhook_update(enable_webhooks, user_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}