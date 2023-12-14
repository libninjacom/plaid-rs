use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::statements_download`].

On request success, this will return a [`String`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsDownloadRequest {
    pub access_token: String,
    pub statement_id: String,
}
impl StatementsDownloadRequest {}
impl FluentRequest<'_, StatementsDownloadRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsDownloadRequest> {
    type Output = httpclient::InMemoryResult<String>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/statements/download";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}