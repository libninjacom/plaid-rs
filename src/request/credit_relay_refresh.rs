use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditRelayRefreshRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayRefreshRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditRelayRefreshResponse> {
        let mut r = self.http_client.client.post("/credit/relay/refresh");
        r = r.json(json!({ "relay_token" : self.relay_token }));
        r = r.json(json!({ "report_type" : self.report_type }));
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
impl<'a> ::std::future::IntoFuture for CreditRelayRefreshRequest<'a> {
    type Output = crate::Result<CreditRelayRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
