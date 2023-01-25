#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let test_clock_id = "your test clock id";
    let response = client.sandbox_transfer_test_clock_get(test_clock_id).await.unwrap();
    println!("{:#?}", response);
}