#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let idempotency_key = "your idempotency key";
    let network = "your network";
    let response = client
        .transfer_ledger_deposit(amount, idempotency_key, network)
        .description("your description")
        .funding_account_id("your funding account id")
        .originator_client_id("your originator client id")
        .await
        .unwrap();
    println!("{:#?}", response);
}