#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let confirmed_hits = &["your confirmed hits"];
    let dismissed_hits = &["your dismissed hits"];
    let entity_watchlist_screening_id = "your entity watchlist screening id";
    let response = client
        .watchlist_screening_entity_review_create(
            confirmed_hits,
            dismissed_hits,
            entity_watchlist_screening_id,
        )
        .comment("your comment")
        .await
        .unwrap();
    println!("{:#?}", response);
}
