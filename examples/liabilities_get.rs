#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .liabilities_get(access_token)
        .options(LiabilitiesGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
