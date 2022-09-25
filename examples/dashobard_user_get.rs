use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let dashboard_user_id = "your dashboard user id";
    let response = client.dashobard_user_get(dashboard_user_id).send().await.unwrap();
    println!("{:#?}", response);
}
