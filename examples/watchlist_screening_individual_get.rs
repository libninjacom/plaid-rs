use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_screening_id = "your watchlist screening id";
    let response = client
        .watchlist_screening_individual_get(watchlist_screening_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
