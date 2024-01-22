use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::statements_list`].

On request success, this will return a [`StatementsListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsListRequest {
    pub access_token: String,
}
impl StatementsListRequest {}
impl FluentRequest<'_, StatementsListRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsListRequest> {
    type Output = httpclient::InMemoryResult<StatementsListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/statements/list";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}