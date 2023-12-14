use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRepaymentListRequest {
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl TransferRepaymentListRequest {}
impl FluentRequest<'_, TransferRepaymentListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_date = Some(end_date);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRepaymentListRequest> {
    type Output = httpclient::InMemoryResult<TransferRepaymentListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/repayment/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}