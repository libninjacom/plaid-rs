use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::BankTransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = BankTransferCreateRequired {
        iso_currency_code: "your iso currency code",
        idempotency_key: "your idempotency key",
        description: "your description",
        access_token: "your access token",
        network: "your network",
        amount: "your amount",
        user: BankTransferUser {
            legal_name: "your legal name".to_owned(),
            email_address: Some("your email address".to_owned()),
            routing_number: Some("your routing number".to_owned()),
        },
        type_: "your type",
        account_id: "your account id",
    };
    let response = client
        .bank_transfer_create(args)
        .ach_class("your ach class")
        .custom_tag("your custom tag")
        .metadata(BankTransferMetadata {})
        .origination_account_id("your origination account id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
