use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let audit_copy_token = "your audit copy token";
    let response = client
        .credit_report_audit_copy_remove(audit_copy_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
