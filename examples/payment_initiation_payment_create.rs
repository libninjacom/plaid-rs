#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let recipient_id = "your recipient id";
    let reference = "your reference";
    let amount = PaymentAmount {
        currency: "your currency".to_owned(),
        value: 1.0,
    };
    let response = client
        .payment_initiation_payment_create(recipient_id, reference, amount)
        .schedule(ExternalPaymentScheduleRequest {
            external_payment_schedule_base: Some(ExternalPaymentScheduleBase {
                adjusted_start_date: Some("your adjusted start date".to_owned()),
                end_date: Some("your end date".to_owned()),
                interval: Some("your interval".to_owned()),
                interval_execution_day: Some(1),
                start_date: Some("your start date".to_owned()),
            }),
        })
        .options(ExternalPaymentOptions {
            bacs: Some(PaymentInitiationOptionalRestrictionBacs {
                recipient_bacs: Some(RecipientBacs {
                    account: Some("your account".to_owned()),
                    sort_code: Some("your sort code".to_owned()),
                }),
            }),
            iban: Some("your iban".to_owned()),
            request_refund_details: Some(true),
            scheme: Some(PaymentScheme(::serde_json::json!({}))),
            wallet_id: Some("your wallet id".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
