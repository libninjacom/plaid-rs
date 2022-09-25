use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let institution_id = "your institution id";
    let initial_products = &["your initial products"];
    let response = client
        .sandbox_public_token_create(institution_id, initial_products)
        .options(SandboxPublicTokenCreateRequestOptions {
            override_username: Some("your override username".to_owned()),
            transactions: Some(SandboxPublicTokenCreateRequestOptionsTransactions {
                start_date: Some("your start date".to_owned()),
                end_date: Some("your end date".to_owned()),
            }),
            webhook: Some("your webhook".to_owned()),
            override_password: Some("your override password".to_owned()),
        })
        .user_token("your user token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
