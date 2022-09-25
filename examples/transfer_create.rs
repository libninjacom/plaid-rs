use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::TransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = TransferCreateRequired {
        amount: "your amount",
        description: "your description",
        user: TransferUserInRequest {
            address: Some(TransferUserAddressInRequest {
                city: Some("your city".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country: Some("your country".to_owned()),
                street: Some("your street".to_owned()),
                region: Some("your region".to_owned()),
            }),
            phone_number: Some("your phone number".to_owned()),
            email_address: Some("your email address".to_owned()),
            legal_name: "your legal name".to_owned(),
        },
        network: "your network",
        type_: "your type",
        ach_class: "your ach class",
        authorization_id: "your authorization id",
    };
    let response = client
        .transfer_create(args)
        .idempotency_key("your idempotency key")
        .access_token("your access token")
        .account_id("your account id")
        .metadata(TransferMetadata {})
        .origination_account_id("your origination account id")
        .iso_currency_code("your iso currency code")
        .payment_profile_id("your payment profile id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
