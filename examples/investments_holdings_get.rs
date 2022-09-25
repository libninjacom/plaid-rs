use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .investments_holdings_get(access_token)
        .options(InvestmentHoldingsGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
