#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let funding_account = TransferFundingAccount {
        access_token: "your access token".to_owned(),
        account_id: "your account id".to_owned(),
    };
    let originator_client_id = "your originator client id";
    let response = client
        .transfer_originator_funding_account_update(
            funding_account,
            originator_client_id,
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}