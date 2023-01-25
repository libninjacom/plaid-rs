#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let audit_copy_token = "your audit copy token";
    let response = client
        .credit_asset_report_freddie_mac_get(audit_copy_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}