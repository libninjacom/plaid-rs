#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let strategy = "your strategy";
    let template_id = "your template id";
    let response = client
        .identity_verification_retry(client_user_id, strategy, template_id)
        .steps(IdentityVerificationRetryRequestStepsObject {
            documentary_verification: true,
            kyc_check: true,
            selfie_check: true,
            verify_sms: true,
        })
        .user(IdentityVerificationRequestUser {
            address: Some(UserAddress {
                city: "your city".to_owned(),
                country: "your country".to_owned(),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: "your street".to_owned(),
                street2: Some("your street 2".to_owned()),
            }),
            date_of_birth: Some(chrono::Utc::now().date_naive()),
            email_address: Some("your email address".to_owned()),
            id_number: Some(UserIdNumber {
                type_: "your type".to_owned(),
                value: "your value".to_owned(),
            }),
            name: Some(IdentityVerificationRequestUserName {
                family_name: "your family name".to_owned(),
                given_name: "your given name".to_owned(),
            }),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}