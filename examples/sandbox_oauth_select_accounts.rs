#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let oauth_state_id = "your oauth state id";
    let accounts = &["your accounts"];
    let response = client
        .sandbox_oauth_select_accounts(oauth_state_id, accounts)
        .await
        .unwrap();
    println!("{:#?}", response);
}