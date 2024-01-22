use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_create_public_token`].

On request success, this will return a [`ItemPublicTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCreatePublicTokenRequest {
    pub access_token: String,
}
impl ItemCreatePublicTokenRequest {}
impl FluentRequest<'_, ItemCreatePublicTokenRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemCreatePublicTokenRequest> {
    type Output = httpclient::InMemoryResult<ItemPublicTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/public_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}