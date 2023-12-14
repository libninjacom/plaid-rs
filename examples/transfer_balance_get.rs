#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_balance_get()
        .originator_client_id("your originator client id")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}