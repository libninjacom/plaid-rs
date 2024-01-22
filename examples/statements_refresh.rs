#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let end_date = chrono::Utc::now().date_naive();
    let start_date = chrono::Utc::now().date_naive();
    let response = client
        .statements_refresh(access_token, end_date, start_date)
        .await
        .unwrap();
    println!("{:#?}", response);
}