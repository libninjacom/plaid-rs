#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::ItemApplicationScopesUpdateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = ItemApplicationScopesUpdateRequired {
        access_token: "your access token",
        application_id: "your application id",
        context: "your context",
        scopes: Scopes {
            accounts: Some(
                vec![
                    AccountAccess { account_product_access :
                    Some(AccountProductAccessNullable { account_product_access :
                    AccountProductAccess { account_data : Some(true), statements :
                    Some(true), tax_documents : Some(true) } }), authorized : Some(true),
                    unique_id : "your unique id".to_owned() }
                ],
            ),
            new_accounts: Some(true),
            product_access: Some(ProductAccess {
                accounts_details_transactions: Some(true),
                accounts_routing_number: Some(true),
                accounts_statements: Some(true),
                accounts_tax_statements: Some(true),
                auth: Some(true),
                customers_profiles: Some(true),
                identity: Some(true),
                statements: Some(true),
                transactions: Some(true),
            }),
        },
    };
    let response = client
        .item_application_scopes_update(args)
        .state("your state")
        .await
        .unwrap();
    println!("{:#?}", response);
}
