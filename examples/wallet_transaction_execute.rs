use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::WalletTransactionExecuteRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = WalletTransactionExecuteRequired {
        idempotency_key: "your idempotency key",
        reference: "your reference",
        wallet_id: "your wallet id",
        amount: WalletTransactionAmount {
            value: 1.0,
            iso_currency_code: "your iso currency code".to_owned(),
        },
        counterparty: WalletTransactionCounterparty {
            name: "your name".to_owned(),
            numbers: WalletTransactionCounterpartyNumbers {
                international: Some(WalletTransactionCounterpartyInternational {
                    iban: Some("your iban".to_owned()),
                }),
                bacs: Some(
                    WalletTransactionCounterpartyBacs(RecipientBacs {
                        account: Some("your account".to_owned()),
                        sort_code: Some("your sort code".to_owned()),
                    }),
                ),
            },
        },
    };
    let response = client.wallet_transaction_execute(args).send().await.unwrap();
    println!("{:#?}", response);
}
