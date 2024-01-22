#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_duplicate_id = "your beacon duplicate id";
    let response = client.beacon_duplicate_get(beacon_duplicate_id).await.unwrap();
    println!("{:#?}", response);
}