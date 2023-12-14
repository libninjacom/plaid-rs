#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let end_date = chrono::Utc::now().date_naive();
    let processor_token = "your processor token";
    let start_date = chrono::Utc::now().date_naive();
    let response = client
        .processor_transactions_get(end_date, processor_token, start_date)
        .options(ProcessorTransactionsGetRequestOptions {
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