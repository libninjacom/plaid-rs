use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::ProcessorBankTransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = ProcessorBankTransferCreateRequired {
        user: BankTransferUser {
            legal_name: "your legal name".to_owned(),
            email_address: Some("your email address".to_owned()),
            routing_number: Some("your routing number".to_owned()),
        },
        idempotency_key: "your idempotency key",
        network: "your network",
        description: "your description",
        amount: "your amount",
        iso_currency_code: "your iso currency code",
        processor_token: "your processor token",
        type_: "your type",
    };
    let response = client
        .processor_bank_transfer_create(args)
        .ach_class("your ach class")
        .custom_tag("your custom tag")
        .metadata(BankTransferMetadata {})
        .origination_account_id("your origination account id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
