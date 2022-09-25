use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let report_tokens = vec![
        ReportToken { report_type : Some("your report type".to_owned()), token :
        Some("your token".to_owned()) }
    ];
    let auditor_id = "your auditor id";
    let response = client
        .credit_audit_copy_token_create(report_tokens, auditor_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
