#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .income_verification_taxforms_get()
        .access_token("your access token")
        .income_verification_id("your income verification id")
        .await
        .unwrap();
    println!("{:#?}", response);
}