#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let payment_id = "your payment id";
    let response = client.create_payment_token(payment_id).await.unwrap();
    println!("{:#?}", response);
}