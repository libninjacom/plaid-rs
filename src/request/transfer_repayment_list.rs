use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_repayment_list`].

On request success, this will return a [`TransferRepaymentListResponse`].*/
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
        Box::pin(async move {
            let url = "/transfer/repayment/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_date {
                r = r.json(json!({ "start_date" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}