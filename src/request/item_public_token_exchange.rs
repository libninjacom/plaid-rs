use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_public_token_exchange`].

On request success, this will return a [`ItemPublicTokenExchangeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    pub public_token: String,
}
impl ItemPublicTokenExchangeRequest {}
impl FluentRequest<'_, ItemPublicTokenExchangeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemPublicTokenExchangeRequest> {
    type Output = httpclient::InMemoryResult<ItemPublicTokenExchangeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/item/public_token/exchange";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}