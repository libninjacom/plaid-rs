#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let account_number = "your account number";
    let routing_number = "your routing number";
    let account_type = "your account type";
    let response = client
        .transfer_migrate_account(account_number, routing_number, account_type)
        .wire_routing_number("your wire routing number")
        .await
        .unwrap();
    println!("{:#?}", response);
}