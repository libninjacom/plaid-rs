/// This wraps third party dependencies that should be
/// re-exported as part of the Plaid API for error handling.
///
/// If the HTTP client library is replaced, this module's name
/// should remain unchanged.
pub mod http {
    pub use serde_json::Error as JsonEncodingError;
    pub use std::string::FromUtf8Error;

    // The only response or body types that are in use in this library are
    // of the "InMemory" variant.
    pub use httpclient::{InMemoryBody as Body, InMemoryResponse as Response};
    pub type Error = httpclient::Error<Body>;
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

// The errors given by this enum are those that the consumer should
// conceivably be able to handle. The errors we emit to the consumer should
// ideally be actionable. In other words, if the error occurred, it's their
// fault and they can fix it.
//
// However, in the case where the error given by httpclient is not actionable from the consumer, we still
// surface them as the `Unknown` variant. These kinds of errors are probably bugs in the Plaid API
// client library code.
//
#[derive(Debug)]
pub enum Error {
    /// A failure due to a known, non-2xx status code.
    ///
    /// Yields the `Response` for the request.
    Http(http::Response),
    /// A failure from an unknown HTTP client library error. This should rarely
    /// happen, but if it does, it is likely a bug in the Plaid API client itself.
    ///
    /// Yields the underlying http client's `http:Error`.
    Unknown(http::Error),
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        match value {
            http::Error::IoError(_)
            | http::Error::Utf8Error(_)
            | http::Error::HttpProtocol(_)
            | http::Error::JsonEncoding(_)
            | http::Error::Custom(_)
            | http::Error::TooManyRedirects => Error::Unknown(value),
            http::Error::HttpError(res) => Error::Http(res),
        }
    }
}
