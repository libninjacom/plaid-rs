#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let report_tokens = vec![
        ReportToken { report_type : Some("your report type".to_owned()), token :
        Some("your token".to_owned()) }
    ];
    let secondary_client_id = "your secondary client id";
    let response = client
        .credit_relay_create(report_tokens, secondary_client_id)
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
