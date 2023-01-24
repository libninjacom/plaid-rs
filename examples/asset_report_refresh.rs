#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let response = client
        .asset_report_refresh(asset_report_token)
        .days_requested(1)
        .options(AssetReportRefreshRequestOptions {
            client_report_id: Some("your client report id".to_owned()),
            user: Some(AssetReportUser {
                client_user_id: Some("your client user id".to_owned()),
                email: Some("your email".to_owned()),
                first_name: Some("your first name".to_owned()),
                last_name: Some("your last name".to_owned()),
                middle_name: Some("your middle name".to_owned()),
                phone_number: Some("your phone number".to_owned()),
                ssn: Some("your ssn".to_owned()),
            }),
            webhook: Some("your webhook".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
