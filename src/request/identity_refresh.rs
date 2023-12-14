use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::identity_refresh`].

On request success, this will return a [`IdentityRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRefreshRequest {
    pub access_token: String,
}
impl IdentityRefreshRequest {}
impl FluentRequest<'_, IdentityRefreshRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IdentityRefreshRequest> {
    type Output = httpclient::InMemoryResult<IdentityRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/identity/refresh";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}