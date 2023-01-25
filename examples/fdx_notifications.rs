#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::FdxNotificationsRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = FdxNotificationsRequired {
        category: "your category",
        notification_id: "your notification id",
        notification_payload: FdxNotificationPayload {
            custom_fields: Some(FdxFiAttribute {
                name: Some("your name".to_owned()),
                value: Some("your value".to_owned()),
            }),
            id: Some("your id".to_owned()),
            id_type: Some("your id type".to_owned()),
        },
        publisher: FdxParty {
            home_uri: Some("your home uri".to_owned()),
            logo_uri: Some("your logo uri".to_owned()),
            name: "your name".to_owned(),
            registered_entity_id: Some("your registered entity id".to_owned()),
            registered_entity_name: Some("your registered entity name".to_owned()),
            registry: Some("your registry".to_owned()),
            type_: "your type".to_owned(),
        },
        sent_on: "your sent on",
        type_: "your type",
    };
    let response = client
        .fdx_notifications(args)
        .severity("your severity")
        .priority("your priority")
        .subscriber(FdxParty {
            home_uri: Some("your home uri".to_owned()),
            logo_uri: Some("your logo uri".to_owned()),
            name: "your name".to_owned(),
            registered_entity_id: Some("your registered entity id".to_owned()),
            registered_entity_name: Some("your registered entity name".to_owned()),
            registry: Some("your registry".to_owned()),
            type_: "your type".to_owned(),
        })
        .url(FdxHateoasLink {
            action: Some("your action".to_owned()),
            href: "your href".to_owned(),
            rel: Some("your rel".to_owned()),
            types: Some(vec!["your types".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}