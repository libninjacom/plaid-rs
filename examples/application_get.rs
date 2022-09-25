use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let application_id = "your application id";
    let response = client.application_get(application_id).send().await.unwrap();
    println!("{:#?}", response);
}
