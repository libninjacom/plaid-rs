use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_application_unlink`].

On request success, this will return a [`ItemApplicationUnlinkResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationUnlinkRequest {
    pub access_token: String,
    pub application_id: String,
}
impl ItemApplicationUnlinkRequest {}
impl FluentRequest<'_, ItemApplicationUnlinkRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemApplicationUnlinkRequest> {
    type Output = httpclient::InMemoryResult<ItemApplicationUnlinkResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/application/unlink";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "application_id" : self.params.application_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}