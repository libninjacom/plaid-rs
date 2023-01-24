#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let item_id = "your item id";
    let webhook = "your webhook";
    let verification_status = "your verification status";
    let response = client
        .sandbox_income_fire_webhook(item_id, webhook, verification_status)
        .user_id("your user id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
