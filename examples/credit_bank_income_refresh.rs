use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client
        .credit_bank_income_refresh(user_token)
        .options(CreditBankIncomeRefreshRequestOptions {
            webhook: Some("your webhook".to_owned()),
            days_requested: Some(1),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
