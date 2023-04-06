#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::PaymentInitiationConsentCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = PaymentInitiationConsentCreateRequired {
        constraints: PaymentInitiationConsentConstraints {
            max_payment_amount: PaymentAmount {
                currency: "your currency".to_owned(),
                value: 1.0,
            },
            periodic_amounts: vec![
                PaymentConsentPeriodicAmount { alignment : "your alignment".to_owned(),
                amount : PaymentAmount { currency : "your currency".to_owned(), value :
                1.0 }, interval : "your interval".to_owned() }
            ],
            valid_date_time: Some(PaymentConsentValidDateTime {
                from: Some(chrono::Utc::now()),
                to: Some(chrono::Utc::now()),
            }),
        },
        recipient_id: "your recipient id",
        reference: "your reference",
        scopes: &["your scopes"],
    };
    let response = client
        .payment_initiation_consent_create(args)
        .options(ExternalPaymentInitiationConsentOptions {
            bacs: Some(RecipientBacs {
                account: Some("your account".to_owned()),
                sort_code: Some("your sort code".to_owned()),
            }),
            iban: Some("your iban".to_owned()),
            request_refund_details: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}