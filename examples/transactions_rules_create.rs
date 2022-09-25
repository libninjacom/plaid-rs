use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let personal_finance_category = "your personal finance category";
    let rule_details = TransactionsRuleDetails {
        query: "your query".to_owned(),
        type_: "your type".to_owned(),
        field: "your field".to_owned(),
    };
    let response = client
        .transactions_rules_create(access_token, personal_finance_category, rule_details)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
