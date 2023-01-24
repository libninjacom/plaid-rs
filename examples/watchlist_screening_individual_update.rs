#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_screening_id = "your watchlist screening id";
    let response = client
        .watchlist_screening_individual_update(watchlist_screening_id)
        .search_terms(UpdateIndividualScreeningRequestSearchTerms {
            country: Some("your country".to_owned()),
            date_of_birth: Some("your date of birth".to_owned()),
            document_number: Some("your document number".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            watchlist_program_id: Some("your watchlist program id".to_owned()),
        })
        .assignee("your assignee")
        .status("your status")
        .client_user_id("your client user id")
        .reset_fields(&["your reset fields"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
