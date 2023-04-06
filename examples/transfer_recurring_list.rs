#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_recurring_list()
        .count(1)
        .end_time(chrono::Utc::now())
        .funding_account_id("your funding account id")
        .offset(1)
        .start_time(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}