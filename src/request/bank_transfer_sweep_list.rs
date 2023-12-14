use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferSweepListRequest {
    pub count: Option<i64>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub origination_account_id: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl BankTransferSweepListRequest {}
impl FluentRequest<'_, BankTransferSweepListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_time = Some(end_time);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_time = Some(start_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferSweepListRequest> {
    type Output = httpclient::InMemoryResult<BankTransferSweepListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/bank_transfer/sweep/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}