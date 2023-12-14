use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_remove`].

On request success, this will return a [`ItemRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRemoveRequest {
    pub access_token: String,
}
impl ItemRemoveRequest {}
impl FluentRequest<'_, ItemRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemRemoveRequest> {
    type Output = httpclient::InMemoryResult<ItemRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/item/remove";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}