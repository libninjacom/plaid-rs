use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferEventListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub bank_transfer_type: Option<BankTransferEventListBankTransferType>,
    pub event_types: Option<Vec<String>>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub direction: Option<BankTransferEventListDirection>,
}
impl<'a> BankTransferEventListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BankTransferEventListResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/event/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bank_transfer_id {
            r = r.json(json!({ "bank_transfer_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bank_transfer_type {
            r = r.json(json!({ "bank_transfer_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.event_types {
            r = r.json(json!({ "event_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.direction {
            r = r.json(json!({ "direction" : unwrapped }));
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
    pub fn bank_transfer_id(mut self, bank_transfer_id: &str) -> Self {
        self.bank_transfer_id = Some(bank_transfer_id.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn bank_transfer_type(
        mut self,
        bank_transfer_type: BankTransferEventListBankTransferType,
    ) -> Self {
        self.bank_transfer_type = Some(bank_transfer_type);
        self
    }
    pub fn event_types(
        mut self,
        event_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .event_types = Some(
            event_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
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
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn direction(mut self, direction: BankTransferEventListDirection) -> Self {
        self.direction = Some(direction);
        self
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferEventListRequest<'a> {
    type Output = httpclient::InMemoryResult<BankTransferEventListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
