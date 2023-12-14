#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let parsing_config = &["your parsing config"];
    let user_token = "your user token";
    let response = client
        .credit_payroll_income_parsing_config_update(parsing_config, user_token)
        .item_id("your item id")
        .await
        .unwrap();
    println!("{:#?}", response);
}