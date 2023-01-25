#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let query = "your query";
    let products = &["your products"];
    let response = client.employers_search(query, products).await.unwrap();
    println!("{:#?}", response);
}