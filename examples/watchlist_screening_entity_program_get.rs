use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_program_id = "your entity watchlist program id";
    let response = client
        .watchlist_screening_entity_program_get(entity_watchlist_program_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
