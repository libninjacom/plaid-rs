#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let audit_copy_token = "your audit copy token";
    let report_tokens = &["your report tokens"];
    let response = client
        .credit_audit_copy_token_update(audit_copy_token, report_tokens)
        .await
        .unwrap();
    println!("{:#?}", response);
}