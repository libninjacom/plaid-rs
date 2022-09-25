use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::TransferIntentCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = TransferIntentCreateRequired {
        mode: "your mode",
        ach_class: "your ach class",
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
    };
    let response = client
        .transfer_intent_create(args)
        .account_id("your account id")
        .origination_account_id("your origination account id")
        .metadata(TransferMetadata {})
        .iso_currency_code("your iso currency code")
        .require_guarantee(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
