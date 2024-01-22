use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_token_webhook_update`].

On request success, this will return a [`ProcessorTokenWebhookUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenWebhookUpdateRequest {
    pub processor_token: String,
    pub webhook: Option<String>,
}
impl ProcessorTokenWebhookUpdateRequest {}
impl FluentRequest<'_, ProcessorTokenWebhookUpdateRequest> {
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenWebhookUpdateRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTokenWebhookUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/webhook/update";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}