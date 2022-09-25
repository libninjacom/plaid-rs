use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_screening_id = "your entity watchlist screening id";
    let response = client
        .watchlist_screening_entity_get(entity_watchlist_screening_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
