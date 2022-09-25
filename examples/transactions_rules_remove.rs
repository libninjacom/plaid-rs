use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let rule_id = "your rule id";
    let response = client
        .transactions_rules_remove(access_token, rule_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
