#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .asset_report_get()
        .asset_report_token("your asset report token")
        .fast_report(true)
        .include_insights(true)
        .options(AssetReportGetRequestOptions {
            days_to_include: Some(1),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}