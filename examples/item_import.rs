#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let products = &["your products"];
    let user_auth = ItemImportRequestUserAuth {
        auth_token: "your auth token".to_owned(),
        user_id: "your user id".to_owned(),
    };
    let response = client
        .item_import(products, user_auth)
        .options(ItemImportRequestOptions {
            webhook: Some("your webhook".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}