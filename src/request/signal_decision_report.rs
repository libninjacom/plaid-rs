use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::signal_decision_report`].

On request success, this will return a [`SignalDecisionReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<String>,
    pub initiated: bool,
    pub payment_method: Option<String>,
}
impl SignalDecisionReportRequest {}
impl FluentRequest<'_, SignalDecisionReportRequest> {
    pub fn amount_instantly_available(
        mut self,
        amount_instantly_available: f64,
    ) -> Self {
        self.params.amount_instantly_available = Some(amount_instantly_available);
        self
    }
    pub fn days_funds_on_hold(mut self, days_funds_on_hold: i64) -> Self {
        self.params.days_funds_on_hold = Some(days_funds_on_hold);
        self
    }
    pub fn decision_outcome(mut self, decision_outcome: &str) -> Self {
        self.params.decision_outcome = Some(decision_outcome.to_owned());
        self
    }
    pub fn payment_method(mut self, payment_method: &str) -> Self {
        self.params.payment_method = Some(payment_method.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalDecisionReportRequest> {
    type Output = httpclient::InMemoryResult<SignalDecisionReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/signal/decision/report";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}