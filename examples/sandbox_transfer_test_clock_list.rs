#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .sandbox_transfer_test_clock_list()
        .count(1)
        .end_virtual_time(chrono::Utc::now())
        .offset(1)
        .start_virtual_time(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}