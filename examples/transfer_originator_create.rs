#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let company_name = "your company name";
    let response = client.transfer_originator_create(company_name).await.unwrap();
    println!("{:#?}", response);
}