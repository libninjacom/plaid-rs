use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let sweep_id = "your sweep id";
    let response = client.bank_transfer_sweep_get(sweep_id).send().await.unwrap();
    println!("{:#?}", response);
}
