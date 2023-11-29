use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxOauthSelectAccountsRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub accounts: Vec<String>,
    pub oauth_state_id: String,
}
impl<'a> SandboxOauthSelectAccountsRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxOauthSelectAccountsResponse> {
        let mut r = self
            .http_client
            .client
            .post("/sandbox/oauth/select_accounts");
        r = r.json(json!({ "accounts" : self.accounts }));
        r = r.json(json!({ "oauth_state_id" : self.oauth_state_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for SandboxOauthSelectAccountsRequest<'a> {
    type Output = crate::Result<SandboxOauthSelectAccountsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
