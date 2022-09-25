use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .payment_initiation_payment_list()
        .count(1)
        .cursor("your cursor")
        .consent_id("your consent id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
