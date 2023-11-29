use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorSignalDecisionReportRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<String>,
    pub initiated: bool,
    pub payment_method: Option<String>,
    pub processor_token: String,
}
impl<'a> ProcessorSignalDecisionReportRequest<'a> {
    pub async fn send(self) -> crate::Result<ProcessorSignalDecisionReportResponse> {
        let mut r = self
            .http_client
            .client
            .post("/processor/signal/decision/report");
        if let Some(ref unwrapped) = self.amount_instantly_available {
            r = r.json(json!({ "amount_instantly_available" : unwrapped }));
        }
        r = r.json(json!({ "client_transaction_id" : self.client_transaction_id }));
        if let Some(ref unwrapped) = self.days_funds_on_hold {
            r = r.json(json!({ "days_funds_on_hold" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.decision_outcome {
            r = r.json(json!({ "decision_outcome" : unwrapped }));
        }
        r = r.json(json!({ "initiated" : self.initiated }));
        if let Some(ref unwrapped) = self.payment_method {
            r = r.json(json!({ "payment_method" : unwrapped }));
        }
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn amount_instantly_available(mut self, amount_instantly_available: f64) -> Self {
        self.amount_instantly_available = Some(amount_instantly_available);
        self
    }
    pub fn days_funds_on_hold(mut self, days_funds_on_hold: i64) -> Self {
        self.days_funds_on_hold = Some(days_funds_on_hold);
        self
    }
    pub fn decision_outcome(mut self, decision_outcome: &str) -> Self {
        self.decision_outcome = Some(decision_outcome.to_owned());
        self
    }
    pub fn payment_method(mut self, payment_method: &str) -> Self {
        self.payment_method = Some(payment_method.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ProcessorSignalDecisionReportRequest<'a> {
    type Output = crate::Result<ProcessorSignalDecisionReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
