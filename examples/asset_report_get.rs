#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let response = client
        .asset_report_get(asset_report_token)
        .fast_report(true)
        .include_insights(true)
        .options(AssetReportGetRequestOptions {
            days_to_include: Some(1),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}