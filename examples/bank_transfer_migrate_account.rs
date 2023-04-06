#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let account_number = "your account number";
    let account_type = "your account type";
    let routing_number = "your routing number";
    let response = client
        .bank_transfer_migrate_account(account_number, account_type, routing_number)
        .wire_routing_number("your wire routing number")
        .await
        .unwrap();
    println!("{:#?}", response);
}