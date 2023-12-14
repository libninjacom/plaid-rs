use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferListRequest {
    pub count: Option<i64>,
    pub direction: Option<BankTransferDirection>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl BankTransferListRequest {}
impl FluentRequest<'_, BankTransferListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn direction(mut self, direction: BankTransferDirection) -> Self {
        self.params.direction = Some(direction);
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
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferListRequest> {
    type Output = httpclient::InMemoryResult<BankTransferListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/bank_transfer/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}