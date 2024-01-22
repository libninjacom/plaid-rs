use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_event_list`].

On request success, this will return a [`BankTransferEventListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventListRequest {
    pub account_id: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub bank_transfer_type: Option<BankTransferEventListBankTransferType>,
    pub count: Option<i64>,
    pub direction: Option<BankTransferEventListDirection>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub event_types: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl BankTransferEventListRequest {}
impl FluentRequest<'_, BankTransferEventListRequest> {
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.params.account_id = Some(account_id.to_owned());
        self
    }
    pub fn bank_transfer_id(mut self, bank_transfer_id: &str) -> Self {
        self.params.bank_transfer_id = Some(bank_transfer_id.to_owned());
        self
    }
    pub fn bank_transfer_type(
        mut self,
        bank_transfer_type: BankTransferEventListBankTransferType,
    ) -> Self {
        self.params.bank_transfer_type = Some(bank_transfer_type);
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn direction(mut self, direction: BankTransferEventListDirection) -> Self {
        self.params.direction = Some(direction);
        self
    }
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_date = Some(end_date);
        self
    }
    pub fn event_types(
        mut self,
        event_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .event_types = Some(
            event_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferEventListRequest> {
    type Output = httpclient::InMemoryResult<BankTransferEventListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/event/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.account_id {
                r = r.json(json!({ "account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bank_transfer_id {
                r = r.json(json!({ "bank_transfer_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bank_transfer_type {
                r = r.json(json!({ "bank_transfer_type" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.direction {
                r = r.json(json!({ "direction" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.event_types {
                r = r.json(json!({ "event_types" : unwrapped }));
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