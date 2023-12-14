#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let item_id = "your item id";
    let webhook = "your webhook";
    let webhook_code = "your webhook code";
    let response = client
        .sandbox_income_fire_webhook(item_id, webhook, webhook_code)
        .user_id("your user id")
        .verification_status("your verification status")
        .await
        .unwrap();
    println!("{:#?}", response);
}