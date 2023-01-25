#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client.item_access_token_invalidate(access_token).await.unwrap();
    println!("{:#?}", response);
}