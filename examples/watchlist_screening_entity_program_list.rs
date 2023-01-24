#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .watchlist_screening_entity_program_list()
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}
