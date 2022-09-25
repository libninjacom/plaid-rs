use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let account_type = "your account type";
    let transactions = vec![
        ClientProvidedRawTransaction { id : "your id".to_owned(), iso_currency_code :
        "your iso currency code".to_owned(), amount : 1.0, description :
        "your description".to_owned() }
    ];
    let response = client
        .transactions_enhance(account_type, transactions)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
