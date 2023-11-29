use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemWebhookUpdateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub webhook: Option<String>,
}
impl<'a> ItemWebhookUpdateRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemWebhookUpdateResponse> {
        let mut r = self.http_client.client.post("/item/webhook/update");
        r = r.json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.json(json!({ "webhook" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ItemWebhookUpdateRequest<'a> {
    type Output = crate::Result<ItemWebhookUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
