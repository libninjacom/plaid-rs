use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_list`].

On request success, this will return a [`SandboxTransferTestClockListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockListRequest {
    pub count: Option<i64>,
    pub end_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub start_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl SandboxTransferTestClockListRequest {}
impl FluentRequest<'_, SandboxTransferTestClockListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn end_virtual_time(
        mut self,
        end_virtual_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.end_virtual_time = Some(end_virtual_time);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    pub fn start_virtual_time(
        mut self,
        start_virtual_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.start_virtual_time = Some(start_virtual_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockListRequest> {
    type Output = httpclient::InMemoryResult<SandboxTransferTestClockListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/transfer/test_clock/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}