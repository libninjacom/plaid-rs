#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let wallet_id = "your wallet id";
    let response = client
        .wallet_transactions_list(wallet_id)
        .cursor("your cursor")
        .count(1)
        .options(WalletTransactionListRequestOptions {
            end_time: Some("your end time".to_owned()),
            start_time: Some("your start time".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}