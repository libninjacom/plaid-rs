#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_payroll_income_precheck()
        .access_tokens(&["your access tokens"])
        .employer(IncomeVerificationPrecheckEmployer {
            address: Some(IncomeVerificationPrecheckEmployerAddress {
            }),
            name: Some("your name".to_owned()),
            tax_id: Some("your tax id".to_owned()),
            url: Some("your url".to_owned()),
        })
        .payroll_institution(IncomeVerificationPrecheckPayrollInstitution {
            name: Some("your name".to_owned()),
        })
        .us_military_info(IncomeVerificationPrecheckMilitaryInfo {
            branch: Some("your branch".to_owned()),
            is_active_duty: Some(true),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}