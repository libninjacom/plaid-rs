use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let verification_status = "your verification status";
    let response = client
        .sandbox_item_set_verification_status(
            access_token,
            account_id,
            verification_status,
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
