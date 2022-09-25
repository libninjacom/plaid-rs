use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let template_id = "your template id";
    let strategy = "your strategy";
    let response = client
        .identity_verification_retry(client_user_id, template_id, strategy)
        .steps(IdentityVerificationRetryRequestStepsObject {
            documentary_verification: true,
            verify_sms: true,
            kyc_check: true,
            selfie_check: true,
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
