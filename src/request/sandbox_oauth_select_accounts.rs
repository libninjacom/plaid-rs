use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_oauth_select_accounts`].

On request success, this will return a [`SandboxOauthSelectAccountsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    pub accounts: Vec<String>,
    pub oauth_state_id: String,
}
impl SandboxOauthSelectAccountsRequest {}
impl FluentRequest<'_, SandboxOauthSelectAccountsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxOauthSelectAccountsRequest> {
    type Output = httpclient::InMemoryResult<SandboxOauthSelectAccountsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/oauth/select_accounts";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "accounts" : self.params.accounts }));
            r = r.json(json!({ "oauth_state_id" : self.params.oauth_state_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}