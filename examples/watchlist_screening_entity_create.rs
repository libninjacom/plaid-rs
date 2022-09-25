use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let search_terms = EntityWatchlistSearchTerms {
        phone_number: Some("your phone number".to_owned()),
        entity_watchlist_program_id: "your entity watchlist program id".to_owned(),
        document_number: Some("your document number".to_owned()),
        country: Some("your country".to_owned()),
        legal_name: "your legal name".to_owned(),
        url: Some("your url".to_owned()),
        email_address: Some("your email address".to_owned()),
    };
    let response = client
        .watchlist_screening_entity_create(search_terms)
        .client_user_id("your client user id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
