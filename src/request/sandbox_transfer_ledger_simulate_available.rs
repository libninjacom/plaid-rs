use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_ledger_simulate_available`].

On request success, this will return a [`SandboxTransferLedgerSimulateAvailableResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferLedgerSimulateAvailableRequest {
    pub test_clock_id: Option<String>,
}
impl SandboxTransferLedgerSimulateAvailableRequest {}
impl FluentRequest<'_, SandboxTransferLedgerSimulateAvailableRequest> {
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferLedgerSimulateAvailableRequest> {
    type Output = httpclient::InMemoryResult<
        SandboxTransferLedgerSimulateAvailableResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/ledger/simulate_available";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(json!({ "test_clock_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}