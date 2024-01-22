use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_advance`].

On request success, this will return a [`SandboxTransferTestClockAdvanceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockAdvanceRequest {
    pub new_virtual_time: chrono::DateTime<chrono::Utc>,
    pub test_clock_id: String,
}
impl SandboxTransferTestClockAdvanceRequest {}
impl FluentRequest<'_, SandboxTransferTestClockAdvanceRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockAdvanceRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferTestClockAdvanceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/advance";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "new_virtual_time" : self.params.new_virtual_time }));
            r = r.json(json!({ "test_clock_id" : self.params.test_clock_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}