use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferIntentGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub transfer_intent_id: String,
}
impl<'a> TransferIntentGetRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferIntentGetResponse> {
        let mut r = self.http_client.client.post("/transfer/intent/get");
        r = r.json(json!({ "transfer_intent_id" : self.transfer_intent_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for TransferIntentGetRequest<'a> {
    type Output = crate::Result<TransferIntentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
