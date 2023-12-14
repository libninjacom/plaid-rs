use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_item_reset_login`].

On request success, this will return a [`SandboxItemResetLoginResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemResetLoginRequest {
    pub access_token: String,
}
impl SandboxItemResetLoginRequest {}
impl FluentRequest<'_, SandboxItemResetLoginRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SandboxItemResetLoginRequest> {
    type Output = httpclient::InMemoryResult<SandboxItemResetLoginResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/item/reset_login";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}