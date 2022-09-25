use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let bank_transfer_id = "your bank transfer id";
    let response = client.bank_transfer_cancel(bank_transfer_id).send().await.unwrap();
    println!("{:#?}", response);
}
