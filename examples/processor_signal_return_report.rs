#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_transaction_id = "your client transaction id";
    let processor_token = "your processor token";
    let return_code = "your return code";
    let response = client
        .processor_signal_return_report(
            client_transaction_id,
            processor_token,
            return_code,
        )
        .returned_at(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}