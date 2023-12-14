use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/sandbox/oauth/select_accounts";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}