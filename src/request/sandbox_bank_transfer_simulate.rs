use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxBankTransferSimulateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
}
impl<'a> SandboxBankTransferSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxBankTransferSimulateResponse> {
        let mut r = self.http_client.client.post("/sandbox/bank_transfer/simulate");
        r = r.json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = r.json(json!({ "event_type" : self.event_type }));
        if let Some(ref unwrapped) = self.failure_reason {
            r = r.json(json!({ "failure_reason" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn failure_reason(mut self, failure_reason: BankTransferFailure) -> Self {
        self.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxBankTransferSimulateRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxBankTransferSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}