use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let search_terms = WatchlistScreeningRequestSearchTerms {
        watchlist_program_id: "your watchlist program id".to_owned(),
        legal_name: "your legal name".to_owned(),
        country: Some("your country".to_owned()),
        date_of_birth: Some("your date of birth".to_owned()),
        document_number: Some("your document number".to_owned()),
    };
    let response = client
        .watchlist_screening_individual_create(search_terms)
        .client_user_id("your client user id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
