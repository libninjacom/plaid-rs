use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::PaymentInitiationConsentCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = PaymentInitiationConsentCreateRequired {
        recipient_id: "your recipient id",
        scopes: &["your scopes"],
        reference: "your reference",
        constraints: PaymentInitiationConsentConstraints {
            max_payment_amount: PaymentConsentMaxPaymentAmount(PaymentAmount {
                currency: "your currency".to_owned(),
                value: 1.0,
            }),
            periodic_amounts: vec![
                PaymentConsentPeriodicAmount { interval : "your interval".to_owned(),
                alignment : "your alignment".to_owned(), amount :
                PaymentConsentPeriodicAmountAmount(PaymentAmount { currency :
                "your currency".to_owned(), value : 1.0 }) }
            ],
            valid_date_time: Some(PaymentConsentValidDateTime {
                from: Some("your from".to_owned()),
                to: Some("your to".to_owned()),
            }),
        },
    };
    let response = client
        .payment_initiation_consent_create(args)
        .options(ExternalPaymentInitiationConsentOptions {
            iban: Some("your iban".to_owned()),
            bacs: Some(
                PaymentInitiationOptionalRestrictionBacs(RecipientBacs {
                    account: Some("your account".to_owned()),
                    sort_code: Some("your sort code".to_owned()),
                }),
            ),
            wallet_id: Some("your wallet id".to_owned()),
            request_refund_details: Some(true),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
