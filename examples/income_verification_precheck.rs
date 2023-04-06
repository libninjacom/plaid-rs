#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .income_verification_precheck()
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
        .transactions_access_token("your transactions access token")
        .transactions_access_tokens(&["your transactions access tokens"])
        .us_military_info(IncomeVerificationPrecheckMilitaryInfo {
            branch: Some("your branch".to_owned()),
            is_active_duty: Some(true),
        })
        .user(IncomeVerificationPrecheckUser {
            email_address: Some("your email address".to_owned()),
            first_name: Some("your first name".to_owned()),
            home_address: Some(SignalAddressData {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            last_name: Some("your last name".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}