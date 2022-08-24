use plaid::model::LinkTokenCreateRequestUser;
use plaid::PlaidClient;
use plaid::request::LinkTokenCreateRequired;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env()
        // Add this middleware to record requests, e.g. for testing.
        // Do not use the middleware in production!
        .with_middleware(httpclient::middleware::LoggerMiddleware::new());
    let item_get = client.link_token_create(LinkTokenCreateRequired {
        client_name: "",
        language: "",
        country_codes: &["US", "CA"],
        user: LinkTokenCreateRequestUser {
            client_user_id: "".to_string(),
            ..Default::default()
        },
    })
        .send()
        .await
        .unwrap();
    println!("{:#?}", item_get);
}