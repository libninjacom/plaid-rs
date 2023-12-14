#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_sweep_list()
        .amount("your amount")
        .count(1)
        .end_date(chrono::Utc::now())
        .funding_account_id("your funding account id")
        .offset(1)
        .originator_client_id("your originator client id")
        .start_date(chrono::Utc::now())
        .status(SweepStatus(serde_json::json!({})))
        .transfer_id("your transfer id")
        .trigger("your trigger")
        .await
        .unwrap();
    println!("{:#?}", response);
}