#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_token = "your link token";
    let response = client
        .link_delivery_create(link_token)
        .options(LinkDeliveryOptions {
            recipient: Some(LinkDeliveryRecipient {
                communication_methods: Some(
                    vec![
                        LinkDeliveryCommunicationMethod { address : Some("your address"
                        .to_owned()), method : Some("your method".to_owned()) }
                    ],
                ),
                first_name: Some("your first name".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}