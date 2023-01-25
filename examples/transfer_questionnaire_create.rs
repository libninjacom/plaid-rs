#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let redirect_uri = "your redirect uri";
    let response = client
        .transfer_questionnaire_create(originator_client_id, redirect_uri)
        .await
        .unwrap();
    println!("{:#?}", response);
}