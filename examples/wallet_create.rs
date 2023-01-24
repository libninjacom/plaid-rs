#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let iso_currency_code = "your iso currency code";
    let response = client.wallet_create(iso_currency_code).await.unwrap();
    println!("{:#?}", response);
}
