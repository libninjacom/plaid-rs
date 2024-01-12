<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/plaid-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/plaid-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/plaid-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/plaid-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/plaid-rs/actions">
        <img src="https://img.shields.io/github/actions/workflow/status/libninjacom/plaid-rs/ci.yaml?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/plaid">
    <img src="https://img.shields.io/crates/d/plaid?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/plaid">
    <img src="https://img.shields.io/crates/v/plaid?style=flat-square" alt="Crates.io" />
</a>

</p>

Plaid client, generated from the OpenAPI spec.

# Usage

```rust
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .item_application_list()
        .access_token("your access token")
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `PLAID_ENV` (Note: this env var should contain the base uri for the plaid service, e.g. https://sandbox.plaid.com, rather than just the env name sandbox/development/production)

* `PLAID_CLIENT_ID`

* `PLAID_SECRET`

* `PLAID_VERSION`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
plaid = ".."
```


# Documentation



* [Client Library Documentation](https://docs.rs/plaid)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
