use plaid::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
        // Add this middleware to record requests, e.g. for testing.
        // Do not use the middleware in production!
        // .with_middleware(httpclient::middleware::RecorderMiddleware::new());
    let access_token ="access-sandbox-b4957595-eae2-4130-9da7-114d14726a62";
    let item_get = client.item_get(access_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", item_get);
}
