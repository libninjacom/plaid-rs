#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let idempotency_key = "your idempotency key";
    let transfer_id = "your transfer id";
    let response = client
        .transfer_refund_create(amount, idempotency_key, transfer_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}