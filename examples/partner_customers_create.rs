#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let company_name = "your company name";
    let is_diligence_attested = true;
    let products = &["your products"];
    let response = client
        .partner_customers_create(company_name, is_diligence_attested, products)
        .create_link_customization(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
