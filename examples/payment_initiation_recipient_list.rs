use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client.payment_initiation_recipient_list().send().await.unwrap();
    println!("{:#?}", response);
}
