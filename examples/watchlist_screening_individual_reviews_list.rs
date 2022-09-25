use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_screening_id = "your watchlist screening id";
    let response = client
        .watchlist_screening_individual_reviews_list(watchlist_screening_id)
        .cursor("your cursor")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
