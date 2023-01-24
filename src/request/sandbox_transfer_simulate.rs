use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferSimulateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
}
impl<'a> SandboxTransferSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxTransferSimulateResponse> {
        let mut r = self.http_client.client.post("/sandbox/transfer/simulate");
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = r.json(json!({ "event_type" : self.event_type }));
        if let Some(ref unwrapped) = self.failure_reason {
            r = r.json(json!({ "failure_reason" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferSimulateRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxTransferSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
