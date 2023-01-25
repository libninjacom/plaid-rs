#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let institution_id = "your institution id";
    let response = client
        .sandbox_processor_token_create(institution_id)
        .options(SandboxProcessorTokenCreateRequestOptions {
            override_password: Some("your override password".to_owned()),
            override_username: Some("your override username".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}