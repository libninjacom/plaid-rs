#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_program_id = "your watchlist program id";
    let response = client
        .watchlist_screening_individual_program_get(watchlist_program_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}