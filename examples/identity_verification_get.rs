#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let identity_verification_id = "your identity verification id";
    let response = client
        .identity_verification_get(identity_verification_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}