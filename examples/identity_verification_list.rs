#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let template_id = "your template id";
    let response = client
        .identity_verification_list(client_user_id, template_id)
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}