use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::ItemApplicationScopesUpdateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = ItemApplicationScopesUpdateRequired {
        context: "your context",
        application_id: "your application id",
        access_token: "your access token",
        scopes: Scopes {
            new_accounts: Some(true),
            accounts: Some(
                vec![
                    AccountAccess { unique_id : "your unique id".to_owned(),
                    account_product_access :
                    Some(AccountProductAccessNullable(AccountProductAccess { account_data
                    : Some(true), statements : Some(true), tax_documents : Some(true)
                    })), authorized : Some(true) }
                ],
            ),
            product_access: Some(ProductAccess {
                accounts_statements: Some(true),
                accounts_routing_number: Some(true),
                transactions: Some(true),
                customers_profiles: Some(true),
                accounts_tax_statements: Some(true),
                auth: Some(true),
                accounts_details_transactions: Some(true),
                identity: Some(true),
                statements: Some(true),
            }),
        },
    };
    let response = client
        .item_application_scopes_update(args)
        .state("your state")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
