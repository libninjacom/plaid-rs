use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxItemFireWebhookRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub webhook_type: Option<String>,
    pub webhook_code: String,
}
impl<'a> SandboxItemFireWebhookRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxItemFireWebhookResponse> {
        let mut r = self.http_client.client.post("/sandbox/item/fire_webhook");
        r = r.json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.webhook_type {
            r = r.json(json!({ "webhook_type" : unwrapped }));
        }
        r = r.json(json!({ "webhook_code" : self.webhook_code }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn webhook_type(mut self, webhook_type: &str) -> Self {
        self.webhook_type = Some(webhook_type.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxItemFireWebhookRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxItemFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}