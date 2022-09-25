use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .item_webhook_update(access_token)
        .webhook("your webhook")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
