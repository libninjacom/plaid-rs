use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_item_fire_webhook`].

On request success, this will return a [`SandboxItemFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    pub access_token: String,
    pub webhook_code: String,
    pub webhook_type: Option<String>,
}
impl SandboxItemFireWebhookRequest {}
impl FluentRequest<'_, SandboxItemFireWebhookRequest> {
    pub fn webhook_type(mut self, webhook_type: &str) -> Self {
        self.params.webhook_type = Some(webhook_type.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SandboxItemFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<SandboxItemFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/item/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "webhook_code" : self.params.webhook_code }));
            if let Some(ref unwrapped) = self.params.webhook_type {
                r = r.json(json!({ "webhook_type" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}