#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_user_id = "your beacon user id";
    let status = "your status";
    let response = client.beacon_user_review(beacon_user_id, status).await.unwrap();
    println!("{:#?}", response);
}