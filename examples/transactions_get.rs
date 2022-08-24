use plaid::model::TransactionsGetRequestOptions;
use plaid::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = std::env::var("ACCESS_TOKEN").unwrap();
    let item_get = client.transactions_get(&access_token, "2022-01-01", "2022-08-04")
        .options(TransactionsGetRequestOptions {
            account_ids: None,
            count: None,
            offset: None,
            include_original_description: None,
            include_personal_finance_category_beta: None,
            include_personal_finance_category: None
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", item_get);
}