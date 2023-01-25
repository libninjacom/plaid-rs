use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRefundCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub transfer_id: String,
    pub amount: String,
    pub idempotency_key: String,
}
impl<'a> TransferRefundCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferRefundCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/refund/create");
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for TransferRefundCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferRefundCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}