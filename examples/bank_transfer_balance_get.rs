#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_balance_get()
        .origination_account_id("your origination account id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
