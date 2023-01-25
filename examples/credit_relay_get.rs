#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let relay_token = "your relay token";
    let report_type = "your report type";
    let response = client.credit_relay_get(relay_token, report_type).await.unwrap();
    println!("{:#?}", response);
}