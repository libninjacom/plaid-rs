use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_transaction_id = "your client transaction id";
    let initiated = true;
    let response = client
        .signal_decision_report(client_transaction_id, initiated)
        .days_funds_on_hold(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
