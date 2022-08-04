//! The [`PlaidClient`] is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]

use serde_json::json;
use httpclient::RequestBuilder;
pub mod model;
pub mod request_model;
use crate::model::*;

impl PlaidClient {
    pub fn from_env() -> PlaidClient {
        let url = format!(
            "https://{}.plaid.com",
            std::env::var("PLAID_ENV")
                .expect("Environment variable PLAID_ENV is not set."),
        );
        PlaidClient::new(&url)
            .with_authentication(PlaidAuthentication::from_env())
    }
}