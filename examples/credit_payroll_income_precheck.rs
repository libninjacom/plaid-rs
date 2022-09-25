use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_payroll_income_precheck()
        .user_token("your user token")
        .access_tokens(&["your access tokens"])
        .employer(IncomeVerificationPrecheckEmployer {
            address: Some(IncomeVerificationPrecheckEmployerAddress {
            }),
            url: Some("your url".to_owned()),
            tax_id: Some("your tax id".to_owned()),
            name: Some("your name".to_owned()),
        })
        .us_military_info(IncomeVerificationPrecheckMilitaryInfo {
            is_active_duty: Some(true),
            branch: Some("your branch".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
