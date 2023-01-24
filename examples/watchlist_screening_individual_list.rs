#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_program_id = "your watchlist program id";
    let response = client
        .watchlist_screening_individual_list(watchlist_program_id)
        .client_user_id("your client user id")
        .status("your status")
        .assignee("your assignee")
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}
