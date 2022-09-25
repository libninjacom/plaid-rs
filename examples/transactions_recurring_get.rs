use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_ids = &["your account ids"];
    let response = client
        .transactions_recurring_get(access_token, account_ids)
        .options(TransactionsRecurringGetRequestOptions {
            include_personal_finance_category: Some(true),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
