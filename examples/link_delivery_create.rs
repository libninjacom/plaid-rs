#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_token = "your link token";
    let delivery_method = "your delivery method";
    let delivery_destination = "your delivery destination";
    let response = client
        .link_delivery_create(link_token, delivery_method, delivery_destination)
        .await
        .unwrap();
    println!("{:#?}", response);
}