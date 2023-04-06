#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_list()
        .count(1)
        .direction(BankTransferDirection(serde_json::json!({})))
        .end_date(chrono::Utc::now())
        .offset(1)
        .origination_account_id("your origination account id")
        .start_date(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}