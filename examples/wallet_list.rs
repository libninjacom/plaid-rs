use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .wallet_list()
        .iso_currency_code("your iso currency code")
        .cursor("your cursor")
        .count(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
