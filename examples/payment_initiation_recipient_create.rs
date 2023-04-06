#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let name = "your name";
    let response = client
        .payment_initiation_recipient_create(name)
        .address(PaymentInitiationAddress {
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: "your postal code".to_owned(),
            street: vec!["your street".to_owned()],
        })
        .bacs(
            Some(RecipientBacs {
                account: Some("your account".to_owned()),
                sort_code: Some("your sort code".to_owned()),
            }),
        )
        .iban("your iban")
        .await
        .unwrap();
    println!("{:#?}", response);
}