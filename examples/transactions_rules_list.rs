use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client.transactions_rules_list(access_token).send().await.unwrap();
    println!("{:#?}", response);
}
