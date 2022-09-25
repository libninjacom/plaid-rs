use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let asset_report_token = "your asset report token";
    let secondary_client_id = "your secondary client id";
    let response = client
        .asset_report_relay_create(asset_report_token, secondary_client_id)
        .webhook("your webhook")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
