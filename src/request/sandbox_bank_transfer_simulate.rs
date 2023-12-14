use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_bank_transfer_simulate`].

On request success, this will return a [`SandboxBankTransferSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateRequest {
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
}
impl SandboxBankTransferSimulateRequest {}
impl FluentRequest<'_, SandboxBankTransferSimulateRequest> {
    pub fn failure_reason(mut self, failure_reason: BankTransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankTransferSimulateRequest> {
    type Output = httpclient::InMemoryResult<SandboxBankTransferSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/bank_transfer/simulate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}