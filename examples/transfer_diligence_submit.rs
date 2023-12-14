#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let originator_diligence = TransferOriginatorDiligence {
        address: TransferOriginatorAddress {
            city: "your city".to_owned(),
            country_code: "your country code".to_owned(),
            postal_code: "your postal code".to_owned(),
            region: "your region".to_owned(),
            street: "your street".to_owned(),
        },
        credit_usage_configuration: Some(TransferCreditUsageConfiguration {
            expected_average_amount: "your expected average amount".to_owned(),
            expected_frequency: "your expected frequency".to_owned(),
            expected_highest_amount: "your expected highest amount".to_owned(),
            expected_monthly_amount: "your expected monthly amount".to_owned(),
            sec_codes: vec!["your sec codes".to_owned()],
        }),
        dba: "your dba".to_owned(),
        debit_usage_configuration: Some(TransferDebitUsageConfiguration {
            expected_average_amount: "your expected average amount".to_owned(),
            expected_frequency: "your expected frequency".to_owned(),
            expected_highest_amount: "your expected highest amount".to_owned(),
            expected_monthly_amount: "your expected monthly amount".to_owned(),
            sec_codes: vec!["your sec codes".to_owned()],
        }),
        funding_account: TransferFundingAccount {
            access_token: "your access token".to_owned(),
            account_id: "your account id".to_owned(),
        },
        naics_code: "your naics code".to_owned(),
        tax_id: "your tax id".to_owned(),
        website: "your website".to_owned(),
    };
    let response = client
        .transfer_diligence_submit(originator_client_id, originator_diligence)
        .await
        .unwrap();
    println!("{:#?}", response);
}