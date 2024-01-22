use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_list`].

On request success, this will return a [`BankTransferListResponse`].*/
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
        Box::pin(async move {
            let url = "/bank_transfer/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.direction {
                r = r.json(json!({ "direction" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(json!({ "origination_account_id" : unwrapped }));
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