use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_fire_webhook`].

On request success, this will return a [`SandboxTransferFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookRequest {
    pub webhook: String,
}
impl SandboxTransferFireWebhookRequest {}
impl FluentRequest<'_, SandboxTransferFireWebhookRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}