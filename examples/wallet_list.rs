#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .wallet_list()
        .count(1)
        .cursor("your cursor")
        .iso_currency_code("your iso currency code")
        .await
        .unwrap();
    println!("{:#?}", response);
}