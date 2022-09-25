use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let transfer_id = "your transfer id";
    let event_type = "your event type";
    let response = client
        .sandbox_transfer_simulate(transfer_id, event_type)
        .failure_reason(TransferFailure {
            ach_return_code: Some("your ach return code".to_owned()),
            description: Some("your description".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
