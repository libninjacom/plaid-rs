use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRepaymentListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> TransferRepaymentListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferRepaymentListResponse> {
        let mut r = self.http_client.client.post("/transfer/repayment/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.json(json!({ "end_date" : unwrapped }));
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
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
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
impl<'a> ::std::future::IntoFuture for TransferRepaymentListRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferRepaymentListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
