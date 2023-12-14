#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::PartnerCustomerCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = PartnerCustomerCreateRequired {
        address: PartnerEndCustomerAddress {
            city: Some("your city".to_owned()),
            country_code: Some("your country code".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            region: Some("your region".to_owned()),
            street: Some("your street".to_owned()),
        },
        application_name: "your application name",
        company_name: "your company name",
        is_bank_addendum_completed: true,
        is_diligence_attested: true,
        legal_entity_name: "your legal entity name",
        website: "your website",
    };
    let response = client
        .partner_customer_create(args)
        .assets_under_management(PartnerEndCustomerAssetsUnderManagement {
            amount: 1.0,
            iso_currency_code: "your iso currency code".to_owned(),
        })
        .billing_contact(PartnerEndCustomerBillingContact {
            email: Some("your email".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
        })
        .client_id("your client id")
        .create_link_customization(true)
        .customer_support_info(PartnerEndCustomerCustomerSupportInfo {
            contact_url: Some("your contact url".to_owned()),
            email: Some("your email".to_owned()),
            link_update_url: Some("your link update url".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .logo("your logo")
        .products(&["your products"])
        .redirect_uris(&["your redirect uris"])
        .secret("your secret")
        .technical_contact(PartnerEndCustomerTechnicalContact {
            email: Some("your email".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}