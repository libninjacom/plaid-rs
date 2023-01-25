#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let test_clock_id = "your test clock id";
    let new_virtual_time = "your new virtual time";
    let response = client
        .sandbox_transfer_test_clock_advance(test_clock_id, new_virtual_time)
        .await
        .unwrap();
    println!("{:#?}", response);
}