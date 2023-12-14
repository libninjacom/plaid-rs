use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferLedgerDepositSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub sweep_id: String,
}
impl SandboxTransferLedgerDepositSimulateRequest {}
impl FluentRequest<'_, SandboxTransferLedgerDepositSimulateRequest> {
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferLedgerDepositSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        SandboxTransferLedgerDepositSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/ledger/deposit/simulate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}