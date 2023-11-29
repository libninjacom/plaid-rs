use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditRelayCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub report_tokens: Vec<String>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditRelayCreateResponse> {
        let mut r = self.http_client.client.post("/credit/relay/create");
        r = r.json(json!({ "report_tokens" : self.report_tokens }));
        r = r.json(json!({ "secondary_client_id" : self.secondary_client_id }));
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
impl<'a> ::std::future::IntoFuture for CreditRelayCreateRequest<'a> {
    type Output = crate::Result<CreditRelayCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
