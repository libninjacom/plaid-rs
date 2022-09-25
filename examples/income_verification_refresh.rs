use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .income_verification_refresh()
        .income_verification_id("your income verification id")
        .access_token("your access token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
