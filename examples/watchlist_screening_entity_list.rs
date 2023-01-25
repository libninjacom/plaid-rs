#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_program_id = "your entity watchlist program id";
    let response = client
        .watchlist_screening_entity_list(entity_watchlist_program_id)
        .client_user_id("your client user id")
        .status("your status")
        .assignee("your assignee")
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}