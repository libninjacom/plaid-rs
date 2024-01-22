use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_relay_refresh`].

On request success, this will return a [`CreditRelayRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayRefreshRequest {
    pub relay_token: String,
    pub report_type: String,
    pub webhook: Option<String>,
}
impl CreditRelayRefreshRequest {}
impl FluentRequest<'_, CreditRelayRefreshRequest> {
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayRefreshRequest> {
    type Output = httpclient::InMemoryResult<CreditRelayRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "relay_token" : self.params.relay_token }));
            r = r.json(json!({ "report_type" : self.params.report_type }));
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}