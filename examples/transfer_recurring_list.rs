#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_recurring_list()
        .start_time("your start time")
        .end_time("your end time")
        .count(1)
        .offset(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}