use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .watchlist_screening_individual_program_list()
        .cursor("your cursor")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
