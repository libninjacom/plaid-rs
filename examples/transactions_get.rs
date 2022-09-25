use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let start_date = "your start date";
    let end_date = "your end date";
    let response = client
        .transactions_get(access_token, start_date, end_date)
        .options(TransactionsGetRequestOptions {
            offset: Some(1),
            include_personal_finance_category: Some(true),
            include_personal_finance_category_beta: Some(true),
            include_original_description: Some(true),
            count: Some(1),
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
