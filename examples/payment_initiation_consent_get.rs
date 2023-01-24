#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let consent_id = "your consent id";
    let response = client.payment_initiation_consent_get(consent_id).await.unwrap();
    println!("{:#?}", response);
}
