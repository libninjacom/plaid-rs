#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let institution_id = "your institution id";
    let initial_products = &["your initial products"];
    let response = client
        .sandbox_public_token_create(institution_id, initial_products)
        .options(SandboxPublicTokenCreateRequestOptions {
            income_verification: Some(SandboxPublicTokenCreateRequestOptionsIncomeVerification {
                bank_income: Some(SandboxPublicTokenCreateRequestIncomeVerificationBankIncome {
                    days_requested: Some(1),
                }),
                income_source_types: Some(vec!["your income source types".to_owned()]),
            }),
            override_password: Some("your override password".to_owned()),
            override_username: Some("your override username".to_owned()),
            transactions: Some(SandboxPublicTokenCreateRequestOptionsTransactions {
                end_date: Some("your end date".to_owned()),
                start_date: Some("your start date".to_owned()),
            }),
            webhook: Some("your webhook".to_owned()),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}