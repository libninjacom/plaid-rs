#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let query = "your query";
    let country_codes = &["your country codes"];
    let response = client
        .institutions_search(query, country_codes)
        .products(&["your products"])
        .options(InstitutionsSearchRequestOptions {
            include_auth_metadata: Some(true),
            include_optional_metadata: Some(true),
            include_payment_initiation_metadata: Some(true),
            oauth: Some(true),
            payment_initiation: Some(InstitutionsSearchPaymentInitiationOptions {
                consent_id: Some("your consent id".to_owned()),
                payment_id: Some("your payment id".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
