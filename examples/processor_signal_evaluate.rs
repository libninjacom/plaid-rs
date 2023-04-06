#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = 1.0;
    let client_transaction_id = "your client transaction id";
    let processor_token = "your processor token";
    let response = client
        .processor_signal_evaluate(amount, client_transaction_id, processor_token)
        .client_user_id("your client user id")
        .default_payment_method("your default payment method")
        .device(SignalDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .is_recurring(true)
        .user(SignalUser {
            address: Some(SignalAddressData {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            name: Some(SignalPersonName {
                family_name: Some("your family name".to_owned()),
                given_name: Some("your given name".to_owned()),
                middle_name: Some("your middle name".to_owned()),
                prefix: Some("your prefix".to_owned()),
                suffix: Some("your suffix".to_owned()),
            }),
            phone_number: Some("your phone number".to_owned()),
        })
        .user_present(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}