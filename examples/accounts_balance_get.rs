#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .accounts_balance_get(access_token)
        .options(AccountsBalanceGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            min_last_updated_datetime: Some(chrono::Utc::now()),
        })
        .payment_details(AccountsBalanceGetRequestPaymentDetails {
            account_id: Some("your account id".to_owned()),
            amount: Some(1.0),
            balance_threshold_percentage: Some(1),
            client_transaction_id: Some("your client transaction id".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}