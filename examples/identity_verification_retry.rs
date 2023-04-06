#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let strategy = "your strategy";
    let template_id = "your template id";
    let response = client
        .identity_verification_retry(client_user_id, strategy, template_id)
        .steps(IdentityVerificationRetryRequestStepsObject {
            documentary_verification: true,
            kyc_check: true,
            selfie_check: true,
            verify_sms: true,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}