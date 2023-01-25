#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_capabilities_get()
        .access_token("your access token")
        .account_id("your account id")
        .payment_profile_token("your payment profile token")
        .await
        .unwrap();
    println!("{:#?}", response);
}