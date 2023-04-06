#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_sweep_list()
        .count(1)
        .end_time(chrono::Utc::now())
        .origination_account_id("your origination account id")
        .start_time(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}