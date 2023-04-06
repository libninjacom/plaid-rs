#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let count = 1;
    let country_codes = &["your country codes"];
    let offset = 1;
    let response = client
        .institutions_get(count, country_codes, offset)
        .options(InstitutionsGetRequestOptions {
            include_auth_metadata: Some(true),
            include_optional_metadata: Some(true),
            include_payment_initiation_metadata: Some(true),
            oauth: Some(true),
            products: Some(vec!["your products".to_owned()]),
            routing_numbers: Some(vec!["your routing numbers".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}