use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::SignalEvaluateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = SignalEvaluateRequired {
        amount: 1.0,
        access_token: "your access token",
        client_transaction_id: "your client transaction id",
        account_id: "your account id",
    };
    let response = client
        .signal_evaluate(args)
        .user_present(true)
        .client_user_id("your client user id")
        .user(SignalUser {
            name: Some(SignalPersonName {
                middle_name: Some("your middle name".to_owned()),
                suffix: Some("your suffix".to_owned()),
                prefix: Some("your prefix".to_owned()),
                family_name: Some("your family name".to_owned()),
                given_name: Some("your given name".to_owned()),
            }),
            address: Some(SignalAddressData {
                street: Some("your street".to_owned()),
                region: Some("your region".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country: Some("your country".to_owned()),
                city: Some("your city".to_owned()),
            }),
            phone_number: Some("your phone number".to_owned()),
            email_address: Some("your email address".to_owned()),
        })
        .device(SignalDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
