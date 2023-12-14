#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let products = &["your products"];
    let response = client
        .processor_token_permissions_set(processor_token, products)
        .await
        .unwrap();
    println!("{:#?}", response);
}