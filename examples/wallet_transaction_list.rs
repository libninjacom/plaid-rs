#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let wallet_id = "your wallet id";
    let response = client
        .wallet_transaction_list(wallet_id)
        .count(1)
        .cursor("your cursor")
        .options(WalletTransactionListRequestOptions {
            end_time: Some(chrono::Utc::now()),
            start_time: Some(chrono::Utc::now()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}