#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let repayment_id = "your repayment id";
    let response = client
        .transfer_repayment_return_list(repayment_id)
        .count(1)
        .offset(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
