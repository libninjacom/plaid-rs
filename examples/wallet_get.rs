use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let wallet_id = "your wallet id";
    let response = client.wallet_get(wallet_id).send().await.unwrap();
    println!("{:#?}", response);
}
