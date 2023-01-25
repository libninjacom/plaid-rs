#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_sweep_list()
        .origination_account_id("your origination account id")
        .start_time("your start time")
        .end_time("your end time")
        .count(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}