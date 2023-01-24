use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SignalDecisionReportRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub initiated: bool,
    pub days_funds_on_hold: Option<i64>,
}
impl<'a> SignalDecisionReportRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SignalDecisionReportResponse> {
        let mut r = self.http_client.client.post("/signal/decision/report");
        r = r.json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.json(json!({ "initiated" : self.initiated }));
        if let Some(ref unwrapped) = self.days_funds_on_hold {
            r = r.json(json!({ "days_funds_on_hold" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn days_funds_on_hold(mut self, days_funds_on_hold: i64) -> Self {
        self.days_funds_on_hold = Some(days_funds_on_hold);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SignalDecisionReportRequest<'a> {
    type Output = httpclient::InMemoryResult<SignalDecisionReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
