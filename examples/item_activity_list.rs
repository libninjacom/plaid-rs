#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .item_activity_list()
        .access_token("your access token")
        .cursor("your cursor")
        .count(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}