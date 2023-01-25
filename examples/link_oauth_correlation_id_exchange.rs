#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_correlation_id = "your link correlation id";
    let response = client
        .link_oauth_correlation_id_exchange(link_correlation_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}