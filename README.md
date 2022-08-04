<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/plaid-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/plaid-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/plaid-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/plaid-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/plaid-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/plaid-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/plaid-openapi">
    <img src="https://img.shields.io/crates/d/plaid-openapi?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/plaid-openapi">
    <img src="https://img.shields.io/crates/v/plaid-openapi?style=flat-square" alt="Crates.io" />
</a>

</p>

# Usage
```rust
use plaid_openapi::PlaidClient;

#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    // Add this middleware to record requests, e.g. for testing.
    // Do not use the middleware in production!
    // .with_middleware(httpclient::middleware::RecorderMiddleware::new());
    let access_token ="access-sandbox-b4957595-eae2-4130-9da7-114d14726a62".to_string();
    let item_get = client.item_get(access_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", item_get);
}
```

This example loads the client from the environment variables, specifically:

* `PLAID_CLIENT_ID`
* `PLAID_SECRET`
* `PLAID_VERSION`
* `PLAID_ENV`: one of `sandbox`, `development`, or `production`

# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
plaid = "2"
```

# [Documentation](https://docs.rs/plaid-openapi/)

# Previous Versions

This library has undergone several iterations due to being a community effort rather than officially supported by Plaid.

- **1.x**: The source code can be found at https://github.com/ammubhave/plaid-rs. 
- **<= 0.2.0**: The source code can be found at https://github.com/nathankot/plaid-rust.
 
# Contributing

Your contribution is highly appreciated. Do not hesitate to open an issue or a pull request. Note that any contribution
submitted for inclusion in the project will be licensed according to the terms given in the project license.

*Library created with [Libninja](https://www.libninja.com).*

