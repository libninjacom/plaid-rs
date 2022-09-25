use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let target_account = DepositSwitchTargetAccount {
        routing_number: "your routing number".to_owned(),
        account_number: "your account number".to_owned(),
        account_name: "your account name".to_owned(),
        account_subtype: "your account subtype".to_owned(),
    };
    let target_user = DepositSwitchTargetUser {
        address: Some(DepositSwitchAddressData {
            country: "your country".to_owned(),
            city: "your city".to_owned(),
            postal_code: "your postal code".to_owned(),
            region: "your region".to_owned(),
            street: "your street".to_owned(),
        }),
        given_name: "your given name".to_owned(),
        phone: "your phone".to_owned(),
        family_name: "your family name".to_owned(),
        email: "your email".to_owned(),
        tax_payer_id: Some("your tax payer id".to_owned()),
    };
    let response = client
        .deposit_switch_alt_create(target_account, target_user)
        .options(DepositSwitchCreateRequestOptions {
            webhook: Some("your webhook".to_owned()),
            transaction_item_access_tokens: Some(
                vec!["your transaction item access tokens".to_owned()],
            ),
        })
        .country_code("your country code")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
