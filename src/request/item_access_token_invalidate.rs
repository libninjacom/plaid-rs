use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_access_token_invalidate`].

On request success, this will return a [`ItemAccessTokenInvalidateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    pub access_token: String,
}
impl ItemAccessTokenInvalidateRequest {}
impl FluentRequest<'_, ItemAccessTokenInvalidateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemAccessTokenInvalidateRequest> {
    type Output = httpclient::InMemoryResult<ItemAccessTokenInvalidateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/item/access_token/invalidate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}