#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let key_id = "your key id";
    let response = client.webhook_verification_key_get(key_id).await.unwrap();
    println!("{:#?}", response);
}
