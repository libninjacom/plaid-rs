use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_repayment_list()
        .start_date("your start date")
        .end_date("your end date")
        .count(1)
        .offset(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
