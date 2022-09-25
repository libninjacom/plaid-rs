use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let response = client
        .processor_apex_processor_token_create(access_token, account_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
