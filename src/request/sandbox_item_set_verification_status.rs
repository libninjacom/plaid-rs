use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_item_set_verification_status`].

On request success, this will return a [`SandboxItemSetVerificationStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusRequest {
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl SandboxItemSetVerificationStatusRequest {}
impl FluentRequest<'_, SandboxItemSetVerificationStatusRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxItemSetVerificationStatusRequest> {
    type Output = httpclient::InMemoryResult<SandboxItemSetVerificationStatusResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/item/set_verification_status";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_id" : self.params.account_id }));
            r = r
                .json(
                    json!({ "verification_status" : self.params.verification_status }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}