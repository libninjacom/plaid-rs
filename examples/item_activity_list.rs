#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .item_activity_list()
        .access_token("your access token")
        .count(1)
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}