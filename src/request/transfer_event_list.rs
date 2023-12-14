use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_event_list`].

On request success, this will return a [`TransferEventListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEventListRequest {
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
impl TransferEventListRequest {}
impl FluentRequest<'_, TransferEventListRequest> {
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.params.account_id = Some(account_id.to_owned());
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
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
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
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
    pub fn sweep_id(mut self, sweep_id: &str) -> Self {
        self.params.sweep_id = Some(sweep_id.to_owned());
        self
    }
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.params.transfer_id = Some(transfer_id.to_owned());
        self
    }
    pub fn transfer_type(
        mut self,
        transfer_type: TransferEventListTransferType,
    ) -> Self {
        self.params.transfer_type = Some(transfer_type);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferEventListRequest> {
    type Output = httpclient::InMemoryResult<TransferEventListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/event/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}