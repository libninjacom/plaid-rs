#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let payment_profile_id = "your payment profile id";
    let response = client.payment_profile_remove(payment_profile_id).await.unwrap();
    println!("{:#?}", response);
}
