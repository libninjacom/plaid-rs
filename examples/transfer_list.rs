#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_list()
        .count(1)
        .end_date(chrono::Utc::now())
        .funding_account_id("your funding account id")
        .offset(1)
        .origination_account_id("your origination account id")
        .originator_client_id("your originator client id")
        .start_date(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}