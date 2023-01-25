#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .sandbox_transfer_test_clock_list()
        .start_virtual_time("your start virtual time")
        .end_virtual_time("your end virtual time")
        .count(1)
        .offset(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}