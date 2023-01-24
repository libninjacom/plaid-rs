#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::TransferAuthorizationCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = TransferAuthorizationCreateRequired {
        ach_class: "your ach class",
        amount: "your amount",
        network: "your network",
        type_: "your type",
        user: TransferAuthorizationUserInRequest {
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
        .transfer_authorization_create(args)
        .access_token("your access token")
        .account_id("your account id")
        .device(TransferAuthorizationDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .origination_account_id("your origination account id")
        .iso_currency_code("your iso currency code")
        .user_present(true)
        .payment_profile_id("your payment profile id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
