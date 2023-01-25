#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let account_ids_to_exclude = &["your account ids to exclude"];
    let response = client
        .asset_report_filter(asset_report_token, account_ids_to_exclude)
        .await
        .unwrap();
    println!("{:#?}", response);
}