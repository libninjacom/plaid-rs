#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_list()
        .start_date("your start date")
        .end_date("your end date")
        .count(1)
        .offset(1)
        .origination_account_id("your origination account id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
