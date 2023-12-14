#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_report_id = "your beacon report id";
    let response = client.beacon_report_get(beacon_report_id).await.unwrap();
    println!("{:#?}", response);
}