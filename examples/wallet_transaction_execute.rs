#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::WalletTransactionExecuteRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = WalletTransactionExecuteRequired {
        amount: WalletTransactionAmount {
            iso_currency_code: "your iso currency code".to_owned(),
            value: 1.0,
        },
        counterparty: WalletTransactionCounterparty {
            name: "your name".to_owned(),
            numbers: WalletTransactionCounterpartyNumbers {
                bacs: Some(RecipientBacs {
                    account: Some("your account".to_owned()),
                    sort_code: Some("your sort code".to_owned()),
                }),
                international: Some(WalletTransactionCounterpartyInternational {
                    iban: Some("your iban".to_owned()),
                }),
            },
        },
        idempotency_key: "your idempotency key",
        reference: "your reference",
        wallet_id: "your wallet id",
    };
    let response = client.wallet_transaction_execute(args).await.unwrap();
    println!("{:#?}", response);
}