use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    pub amount: Option<String>,
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<SweepStatus>,
    pub transfer_id: Option<String>,
    pub trigger: Option<String>,
}
impl TransferSweepListRequest {}
impl FluentRequest<'_, TransferSweepListRequest> {
    pub fn amount(mut self, amount: &str) -> Self {
        self.params.amount = Some(amount.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_date = Some(end_date);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
    pub fn status(mut self, status: SweepStatus) -> Self {
        self.params.status = Some(status);
        self
    }
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.params.transfer_id = Some(transfer_id.to_owned());
        self
    }
    pub fn trigger(mut self, trigger: &str) -> Self {
        self.params.trigger = Some(trigger.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferSweepListRequest> {
    type Output = httpclient::InMemoryResult<TransferSweepListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/sweep/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}