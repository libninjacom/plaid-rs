#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_event_list()
        .account_id("your account id")
        .count(1)
        .end_date(chrono::Utc::now())
        .event_types(&["your event types"])
        .funding_account_id("your funding account id")
        .offset(1)
        .origination_account_id("your origination account id")
        .originator_client_id("your originator client id")
        .start_date(chrono::Utc::now())
        .sweep_id("your sweep id")
        .transfer_id("your transfer id")
        .transfer_type(TransferEventListTransferType(serde_json::json!({})))
        .await
        .unwrap();
    println!("{:#?}", response);
}