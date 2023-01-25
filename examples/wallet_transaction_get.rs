#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transaction_id = "your transaction id";
    let response = client.wallet_transaction_get(transaction_id).await.unwrap();
    println!("{:#?}", response);
}