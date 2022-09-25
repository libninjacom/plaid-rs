use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .identity_match(access_token)
        .user(IdentityMatchUser {
            phone_number: Some("your phone number".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            email_address: Some("your email address".to_owned()),
            address: Some(
                AddressDataNullable(AddressData {
                    region: Some("your region".to_owned()),
                    postal_code: Some("your postal code".to_owned()),
                    country: Some("your country".to_owned()),
                    street: "your street".to_owned(),
                    city: "your city".to_owned(),
                }),
            ),
        })
        .options(IdentityMatchRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
