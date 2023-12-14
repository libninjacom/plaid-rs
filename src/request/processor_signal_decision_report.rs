use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalDecisionReportRequest {
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<String>,
    pub initiated: bool,
    pub payment_method: Option<String>,
    pub processor_token: String,
}
impl ProcessorSignalDecisionReportRequest {}
impl FluentRequest<'_, ProcessorSignalDecisionReportRequest> {
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalDecisionReportRequest> {
    type Output = httpclient::InMemoryResult<ProcessorSignalDecisionReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/signal/decision/report";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}