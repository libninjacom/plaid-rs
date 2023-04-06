#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let account_ids_to_exclude = &["your account ids to exclude"];
    let asset_report_token = "your asset report token";
    let response = client
        .asset_report_filter(account_ids_to_exclude, asset_report_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}