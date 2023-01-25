#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transfer_id = "your transfer id";
    let amount = "your amount";
    let idempotency_key = "your idempotency key";
    let response = client
        .transfer_refund_create(transfer_id, amount, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}