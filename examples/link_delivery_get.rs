#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_delivery_session_id = "your link delivery session id";
    let response = client.link_delivery_get(link_delivery_session_id).await.unwrap();
    println!("{:#?}", response);
}