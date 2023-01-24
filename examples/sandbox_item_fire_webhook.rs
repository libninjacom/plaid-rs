#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let webhook_code = "your webhook code";
    let response = client
        .sandbox_item_fire_webhook(access_token, webhook_code)
        .webhook_type("your webhook type")
        .await
        .unwrap();
    println!("{:#?}", response);
}
