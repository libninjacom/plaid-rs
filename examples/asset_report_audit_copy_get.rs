use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let audit_copy_token = "your audit copy token";
    let response = client
        .asset_report_audit_copy_get(audit_copy_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
