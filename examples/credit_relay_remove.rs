use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let relay_token = "your relay token";
    let response = client.credit_relay_remove(relay_token).send().await.unwrap();
    println!("{:#?}", response);
}
