use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::link_token_get`].

On request success, this will return a [`LinkTokenGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenGetRequest {
    pub link_token: String,
}
impl LinkTokenGetRequest {}
impl FluentRequest<'_, LinkTokenGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkTokenGetRequest> {
    type Output = httpclient::InMemoryResult<LinkTokenGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/link/token/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}