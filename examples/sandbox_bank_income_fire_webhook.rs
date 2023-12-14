#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let webhook_code = "your webhook code";
    let webhook_fields = SandboxBankIncomeWebhookFireRequestWebhookFields {
        bank_income_refresh_complete_result: Some(
            "your bank income refresh complete result".to_owned(),
        ),
        user_id: "your user id".to_owned(),
    };
    let response = client
        .sandbox_bank_income_fire_webhook(webhook_code, webhook_fields)
        .webhook_override("your webhook override")
        .await
        .unwrap();
    println!("{:#?}", response);
}