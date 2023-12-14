#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .sandbox_transfer_sweep_simulate()
        .test_clock_id("your test clock id")
        .await
        .unwrap();
    println!("{:#?}", response);
}