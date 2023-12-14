use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_bank_transfer_fire_webhook`].

On request success, this will return a [`SandboxBankTransferFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookRequest {
    pub webhook: String,
}
impl SandboxBankTransferFireWebhookRequest {}
impl FluentRequest<'_, SandboxBankTransferFireWebhookRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankTransferFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<SandboxBankTransferFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/bank_transfer/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}