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
        .investments_transactions_get(access_token, end_date, start_date)
        .options(InvestmentsTransactionsGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            count: Some(1),
            offset: Some(1),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}