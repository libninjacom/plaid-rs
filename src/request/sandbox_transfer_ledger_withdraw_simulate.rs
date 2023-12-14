use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferLedgerWithdrawSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub sweep_id: String,
}
impl SandboxTransferLedgerWithdrawSimulateRequest {}
impl FluentRequest<'_, SandboxTransferLedgerWithdrawSimulateRequest> {
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferLedgerWithdrawSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        SandboxTransferLedgerWithdrawSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/ledger/withdraw/simulate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}