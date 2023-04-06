#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::TransferRecurringCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = TransferRecurringCreateRequired {
        access_token: "your access token",
        account_id: "your account id",
        amount: "your amount",
        description: "your description",
        device: TransferDevice {
            ip_address: "your ip address".to_owned(),
            user_agent: "your user agent".to_owned(),
        },
        idempotency_key: "your idempotency key",
        network: "your network",
        schedule: TransferRecurringSchedule {
            end_date: Some(chrono::Utc::now().date()),
            interval_count: 1,
            interval_execution_day: 1,
            interval_unit: "your interval unit".to_owned(),
            start_date: chrono::Utc::now().date(),
        },
        type_: "your type",
        user: TransferUserInRequest {
            address: Some(TransferUserAddressInRequest {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            legal_name: "your legal name".to_owned(),
            phone_number: Some("your phone number".to_owned()),
        },
    };
    let response = client
        .transfer_recurring_create(args)
        .ach_class("your ach class")
        .funding_account_id("your funding account id")
        .iso_currency_code("your iso currency code")
        .test_clock_id("your test clock id")
        .user_present(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}