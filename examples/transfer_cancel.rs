use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transfer_id = "your transfer id";
    let response = client.transfer_cancel(transfer_id).send().await.unwrap();
    println!("{:#?}", response);
}
