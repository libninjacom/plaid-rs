use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferTestClockListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: Option<i64>,
    pub end_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub start_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl<'a> SandboxTransferTestClockListRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxTransferTestClockListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/sandbox/transfer/test_clock/list");
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_virtual_time {
            r = r.json(json!({ "end_virtual_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_virtual_time {
            r = r.json(json!({ "start_virtual_time" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn end_virtual_time(mut self, end_virtual_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_virtual_time = Some(end_virtual_time);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn start_virtual_time(mut self, start_virtual_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_virtual_time = Some(start_virtual_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferTestClockListRequest<'a> {
    type Output = crate::Result<SandboxTransferTestClockListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
