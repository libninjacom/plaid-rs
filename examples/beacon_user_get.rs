#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_user_id = "your beacon user id";
    let response = client.beacon_user_get(beacon_user_id).await.unwrap();
    println!("{:#?}", response);
}