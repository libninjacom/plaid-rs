use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .dashboard_user_list()
        .cursor("your cursor")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
