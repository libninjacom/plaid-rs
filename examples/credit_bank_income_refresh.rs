#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client
        .credit_bank_income_refresh(user_token)
        .options(CreditBankIncomeRefreshRequestOptions {
            days_requested: Some(1),
            webhook: Some("your webhook".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
