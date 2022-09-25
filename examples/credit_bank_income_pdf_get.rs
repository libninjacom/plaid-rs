use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client.credit_bank_income_pdf_get(user_token).send().await.unwrap();
    println!("{:#?}", response);
}
