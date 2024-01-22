use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_refund_simulate`].

On request success, this will return a [`SandboxTransferRefundSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferRefundSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub refund_id: String,
    pub test_clock_id: Option<String>,
}
impl SandboxTransferRefundSimulateRequest {}
impl FluentRequest<'_, SandboxTransferRefundSimulateRequest> {
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferRefundSimulateRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferRefundSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/refund/simulate";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "event_type" : self.params.event_type }));
            if let Some(ref unwrapped) = self.params.failure_reason {
                r = r.json(json!({ "failure_reason" : unwrapped }));
            }
            r = r.json(json!({ "refund_id" : self.params.refund_id }));
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(json!({ "test_clock_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}