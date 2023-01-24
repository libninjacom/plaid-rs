#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let consent_id = "your consent id";
    let amount = PaymentAmount {
        currency: "your currency".to_owned(),
        value: 1.0,
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .payment_initiation_consent_payment_execute(consent_id, amount, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}
