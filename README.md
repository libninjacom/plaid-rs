# `plaid-openapi`

```rust
use plaid_openapi::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token ="access-sandbox-b4957595-eae2-4130-9da7-114d14726a62".to_string();
    let item_get = client.item_get(access_token)
        .await
        .unwrap();
    println!("{:#?}", item_get);
}
```

`plaid-openapi` is a feature-complete, human, async client library for the Plaid API, generated from Plaid's OpenAPI spec using [`openapi-client-generator`](https://github.com/kurtbuilds/openapi-client-generator).

# Contributing

Contributions are welcome!