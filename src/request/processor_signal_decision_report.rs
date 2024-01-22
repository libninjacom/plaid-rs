use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_signal_decision_report`].

On request success, this will return a [`ProcessorSignalDecisionReportResponse`].*/
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
        Box::pin(async move {
            let url = "/processor/signal/decision/report";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.amount_instantly_available {
                r = r.json(json!({ "amount_instantly_available" : unwrapped }));
            }
            r = r
                .json(
                    json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.days_funds_on_hold {
                r = r.json(json!({ "days_funds_on_hold" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.decision_outcome {
                r = r.json(json!({ "decision_outcome" : unwrapped }));
            }
            r = r.json(json!({ "initiated" : self.params.initiated }));
            if let Some(ref unwrapped) = self.params.payment_method {
                r = r.json(json!({ "payment_method" : unwrapped }));
            }
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}