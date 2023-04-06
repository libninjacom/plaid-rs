#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let idempotency_key = "your idempotency key";
    let payment_id = "your payment id";
    let reference = "your reference";
    let response = client
        .payment_initiation_payment_reverse(idempotency_key, payment_id, reference)
        .amount(PaymentAmountNullable {
            currency: "your currency".to_owned(),
            value: 1.0,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}