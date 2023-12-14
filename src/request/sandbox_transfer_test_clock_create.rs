use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockCreateRequest {
    pub virtual_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl SandboxTransferTestClockCreateRequest {}
impl FluentRequest<'_, SandboxTransferTestClockCreateRequest> {
    pub fn virtual_time(mut self, virtual_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.virtual_time = Some(virtual_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockCreateRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferTestClockCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/test_clock/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}