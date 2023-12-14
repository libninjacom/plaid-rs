#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let response = client.transactions_user_insights_get(client_user_id).await.unwrap();
    println!("{:#?}", response);
}