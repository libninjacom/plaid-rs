use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client
        .processor_balance_get(processor_token)
        .options(ProcessorBalanceGetRequestOptions {
            min_last_updated_datetime: Some("your min last updated datetime".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
