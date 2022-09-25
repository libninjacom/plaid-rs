use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::IdentityVerificationCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = IdentityVerificationCreateRequired {
        template_id: "your template id",
        gave_consent: true,
        user: IdentityVerificationRequestUser {
            email_address: Some("your email address".to_owned()),
            name: Some(UserName {
                given_name: "your given name".to_owned(),
                family_name: "your family name".to_owned(),
            }),
            date_of_birth: Some("your date of birth".to_owned()),
            address: Some(UserAddress {
                street: "your street".to_owned(),
                street2: Some("your street 2".to_owned()),
                city: "your city".to_owned(),
                region: "your region".to_owned(),
                country: "your country".to_owned(),
                postal_code: "your postal code".to_owned(),
            }),
            id_number: Some(UserIdNumber {
                type_: "your type".to_owned(),
                value: "your value".to_owned(),
            }),
            phone_number: Some("your phone number".to_owned()),
            client_user_id: "your client user id".to_owned(),
        },
        is_shareable: true,
    };
    let response = client
        .identity_verification_create(args)
        .is_idempotent(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
