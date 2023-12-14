#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let response = client
        .transfer_capabilities_get(access_token, account_id)
        .payment_profile_token("your payment profile token")
        .await
        .unwrap();
    println!("{:#?}", response);
}