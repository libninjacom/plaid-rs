use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_signal_return_report`].

On request success, this will return a [`ProcessorSignalReturnReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalReturnReportRequest {
    pub client_transaction_id: String,
    pub processor_token: String,
    pub return_code: String,
    pub returned_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl ProcessorSignalReturnReportRequest {}
impl FluentRequest<'_, ProcessorSignalReturnReportRequest> {
    pub fn returned_at(mut self, returned_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.returned_at = Some(returned_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalReturnReportRequest> {
    type Output = httpclient::InMemoryResult<ProcessorSignalReturnReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/signal/return/report";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = r.json(json!({ "return_code" : self.params.return_code }));
            if let Some(ref unwrapped) = self.params.returned_at {
                r = r.json(json!({ "returned_at" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}