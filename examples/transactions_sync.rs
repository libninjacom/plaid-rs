#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .transactions_sync(access_token)
        .cursor("your cursor")
        .count(1)
        .options(TransactionsSyncRequestOptions {
            include_logo_and_counterparty_beta: Some(true),
            include_original_description: Some(true),
            include_personal_finance_category: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}