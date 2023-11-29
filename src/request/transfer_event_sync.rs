use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferEventSyncRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> TransferEventSyncRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferEventSyncResponse> {
        let mut r = self.http_client.client.post("/transfer/event/sync");
        r = r.json(json!({ "after_id" : self.after_id }));
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferEventSyncRequest<'a> {
    type Output = crate::Result<TransferEventSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
