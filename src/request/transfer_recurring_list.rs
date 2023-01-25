use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRecurringListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> TransferRecurringListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferRecurringListResponse> {
        let mut r = self.http_client.client.post("/transfer/recurring/list");
        if let Some(ref unwrapped) = self.start_time {
            r = r.json(json!({ "start_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.json(json!({ "end_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn start_time(mut self, start_time: &str) -> Self {
        self.start_time = Some(start_time.to_owned());
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferRecurringListRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferRecurringListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}