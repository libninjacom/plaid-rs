#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let account_type = "your account type";
    let transactions = vec![
        ClientProvidedTransaction { amount : 1.0, date_posted : Some("your date posted"
        .to_owned()), description : "your description".to_owned(), direction :
        "your direction".to_owned(), id : "your id".to_owned(), iso_currency_code :
        "your iso currency code".to_owned(), location :
        Some(ClientProvidedTransactionLocation { address : Some("your address"
        .to_owned()), city : Some("your city".to_owned()), country : Some("your country"
        .to_owned()), postal_code : Some("your postal code".to_owned()), region :
        Some("your region".to_owned()) }), mcc : Some("your mcc".to_owned()) }
    ];
    let response = client
        .transactions_enrich(account_type, transactions)
        .options(TransactionsEnrichRequestOptions {
            include_legacy_category: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}