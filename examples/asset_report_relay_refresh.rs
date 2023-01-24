#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_relay_token = "your asset relay token";
    let response = client
        .asset_report_relay_refresh(asset_relay_token)
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
