#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let application_id = "your application id";
    let response = client
        .item_application_unlink(access_token, application_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}