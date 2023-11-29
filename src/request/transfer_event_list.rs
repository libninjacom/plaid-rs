use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferEventListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub account_id: Option<String>,
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub event_types: Option<Vec<String>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub sweep_id: Option<String>,
    pub transfer_id: Option<String>,
    pub transfer_type: Option<TransferEventListTransferType>,
}
impl<'a> TransferEventListRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferEventListResponse> {
        let mut r = self.http_client.client.post("/transfer/event/list");
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.event_types {
            r = r.json(json!({ "event_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.originator_client_id {
            r = r.json(json!({ "originator_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sweep_id {
            r = r.json(json!({ "sweep_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer_id {
            r = r.json(json!({ "transfer_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer_type {
            r = r.json(json!({ "transfer_type" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }
    pub fn event_types(mut self, event_types: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.event_types = Some(
            event_types
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.funding_account_id = Some(funding_account_id.to_owned());
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
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }
    pub fn sweep_id(mut self, sweep_id: &str) -> Self {
        self.sweep_id = Some(sweep_id.to_owned());
        self
    }
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.transfer_id = Some(transfer_id.to_owned());
        self
    }
    pub fn transfer_type(mut self, transfer_type: TransferEventListTransferType) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferEventListRequest<'a> {
    type Output = crate::Result<TransferEventListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
