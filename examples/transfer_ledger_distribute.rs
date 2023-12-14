#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::TransferLedgerDistributeRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = TransferLedgerDistributeRequired {
        amount: "your amount",
        from_client_id: "your from client id",
        idempotency_key: "your idempotency key",
        to_client_id: "your to client id",
    };
    let response = client
        .transfer_ledger_distribute(args)
        .description("your description")
        .await
        .unwrap();
    println!("{:#?}", response);
}