use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .employment_verification_get(access_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
