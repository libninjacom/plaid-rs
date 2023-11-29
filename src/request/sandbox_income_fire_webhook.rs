use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxIncomeFireWebhookRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub item_id: String,
    pub user_id: Option<String>,
    pub verification_status: String,
    pub webhook: String,
}
impl<'a> SandboxIncomeFireWebhookRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxIncomeFireWebhookResponse> {
        let mut r = self.http_client.client.post("/sandbox/income/fire_webhook");
        r = r.json(json!({ "item_id" : self.item_id }));
        if let Some(ref unwrapped) = self.user_id {
            r = r.json(json!({ "user_id" : unwrapped }));
        }
        r = r.json(json!({ "verification_status" : self.verification_status }));
        r = r.json(json!({ "webhook" : self.webhook }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxIncomeFireWebhookRequest<'a> {
    type Output = crate::Result<SandboxIncomeFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
