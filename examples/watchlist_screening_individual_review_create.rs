use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let confirmed_hits = &["your confirmed hits"];
    let dismissed_hits = &["your dismissed hits"];
    let watchlist_screening_id = "your watchlist screening id";
    let response = client
        .watchlist_screening_individual_review_create(
            confirmed_hits,
            dismissed_hits,
            watchlist_screening_id,
        )
        .comment("your comment")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
