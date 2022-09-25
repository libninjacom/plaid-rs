use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let response = client.user_create(client_user_id).send().await.unwrap();
    println!("{:#?}", response);
}
