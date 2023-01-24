#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let institution_id = "your institution id";
    let country_codes = &["your country codes"];
    let response = client
        .institutions_get_by_id(institution_id, country_codes)
        .options(InstitutionsGetByIdRequestOptions {
            include_auth_metadata: Some(true),
            include_optional_metadata: Some(true),
            include_payment_initiation_metadata: Some(true),
            include_status: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
