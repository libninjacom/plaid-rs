#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_event_list()
        .account_id("your account id")
        .bank_transfer_id("your bank transfer id")
        .bank_transfer_type(BankTransferEventListBankTransferType(serde_json::json!({})))
        .count(1)
        .direction(BankTransferEventListDirection(serde_json::json!({})))
        .end_date(chrono::Utc::now())
        .event_types(&["your event types"])
        .offset(1)
        .origination_account_id("your origination account id")
        .start_date(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}