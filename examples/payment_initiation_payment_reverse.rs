#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let payment_id = "your payment id";
    let idempotency_key = "your idempotency key";
    let reference = "your reference";
    let response = client
        .payment_initiation_payment_reverse(payment_id, idempotency_key, reference)
        .amount(PaymentAmountNullable {
            currency: "your currency".to_owned(),
            value: 1.0,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}