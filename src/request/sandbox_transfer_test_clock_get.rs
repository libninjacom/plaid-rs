use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_get`].

On request success, this will return a [`SandboxTransferTestClockGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockGetRequest {
    pub test_clock_id: String,
}
impl SandboxTransferTestClockGetRequest {}
impl FluentRequest<'_, SandboxTransferTestClockGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockGetRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferTestClockGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "test_clock_id" : self.params.test_clock_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}