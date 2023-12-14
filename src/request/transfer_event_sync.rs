use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_event_sync`].

On request success, this will return a [`TransferEventSyncResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEventSyncRequest {
    pub after_id: i64,
    pub count: Option<i64>,
}
impl TransferEventSyncRequest {}
impl FluentRequest<'_, TransferEventSyncRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferEventSyncRequest> {
    type Output = httpclient::InMemoryResult<TransferEventSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/event/sync";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}