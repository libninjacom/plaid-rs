use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_event_sync`].

On request success, this will return a [`BankTransferEventSyncResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventSyncRequest {
    pub after_id: i64,
    pub count: Option<i64>,
}
impl BankTransferEventSyncRequest {}
impl FluentRequest<'_, BankTransferEventSyncRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferEventSyncRequest> {
    type Output = httpclient::InMemoryResult<BankTransferEventSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/event/sync";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "after_id" : self.params.after_id }));
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}