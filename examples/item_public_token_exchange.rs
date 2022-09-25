use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let public_token = "your public token";
    let response = client.item_public_token_exchange(public_token).send().await.unwrap();
    println!("{:#?}", response);
}
