#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let end_date = chrono::Utc::now().date();
    let start_date = chrono::Utc::now().date();
    let response = client
        .transactions_get(access_token, end_date, start_date)
        .options(TransactionsGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            count: Some(1),
            include_logo_and_counterparty_beta: Some(true),
            include_original_description: Some(true),
            include_personal_finance_category: Some(true),
            include_personal_finance_category_beta: Some(true),
            offset: Some(1),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}