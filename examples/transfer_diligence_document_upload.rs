#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let file = "your file";
    let originator_client_id = "your originator client id";
    let purpose = "your purpose";
    let response = client
        .transfer_diligence_document_upload(file, originator_client_id, purpose)
        .await
        .unwrap();
    println!("{:#?}", response);
}