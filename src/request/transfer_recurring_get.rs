use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRecurringGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub recurring_transfer_id: String,
}
impl<'a> TransferRecurringGetRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferRecurringGetResponse> {
        let mut r = self.http_client.client.post("/transfer/recurring/get");
        r = r.json(json!({ "recurring_transfer_id" : self.recurring_transfer_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for TransferRecurringGetRequest<'a> {
    type Output = crate::Result<TransferRecurringGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
