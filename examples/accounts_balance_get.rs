#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .accounts_balance_get(access_token)
        .options(AccountsBalanceGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            min_last_updated_datetime: Some("your min last updated datetime".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
