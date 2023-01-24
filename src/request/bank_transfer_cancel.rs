use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferCancelRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferCancelRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<BankTransferCancelResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/cancel");
        r = r.json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferCancelRequest<'a> {
    type Output = httpclient::InMemoryResult<BankTransferCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
