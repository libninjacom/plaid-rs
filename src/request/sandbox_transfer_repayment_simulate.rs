use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_repayment_simulate`].

On request success, this will return a [`SandboxTransferRepaymentSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateRequest {}
impl SandboxTransferRepaymentSimulateRequest {}
impl FluentRequest<'_, SandboxTransferRepaymentSimulateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferRepaymentSimulateRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferRepaymentSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/repayment/simulate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}