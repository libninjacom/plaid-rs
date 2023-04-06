#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_transaction_id = "your client transaction id";
    let initiated = true;
    let processor_token = "your processor token";
    let response = client
        .processor_signal_decision_report(
            client_transaction_id,
            initiated,
            processor_token,
        )
        .amount_instantly_available(1.0)
        .days_funds_on_hold(1)
        .decision_outcome("your decision outcome")
        .payment_method("your payment method")
        .await
        .unwrap();
    println!("{:#?}", response);
}