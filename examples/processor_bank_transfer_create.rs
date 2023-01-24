#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::ProcessorBankTransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = ProcessorBankTransferCreateRequired {
        amount: "your amount",
        description: "your description",
        idempotency_key: "your idempotency key",
        iso_currency_code: "your iso currency code",
        network: "your network",
        processor_token: "your processor token",
        type_: "your type",
        user: BankTransferUser {
            email_address: Some("your email address".to_owned()),
            legal_name: "your legal name".to_owned(),
            routing_number: Some("your routing number".to_owned()),
        },
    };
    let response = client
        .processor_bank_transfer_create(args)
        .ach_class("your ach class")
        .custom_tag("your custom tag")
        .metadata(BankTransferMetadata {})
        .origination_account_id("your origination account id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
