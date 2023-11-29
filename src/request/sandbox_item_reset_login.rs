use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxItemResetLoginRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> SandboxItemResetLoginRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxItemResetLoginResponse> {
        let mut r = self.http_client.client.post("/sandbox/item/reset_login");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for SandboxItemResetLoginRequest<'a> {
    type Output = crate::Result<SandboxItemResetLoginResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
