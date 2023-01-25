#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let webhook = "your webhook";
    let response = client
        .income_verification_create(webhook)
        .precheck_id("your precheck id")
        .options(IncomeVerificationCreateRequestOptions {
            access_tokens: Some(vec!["your access tokens".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}