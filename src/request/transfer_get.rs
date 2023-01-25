use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub transfer_id: String,
}
impl<'a> TransferGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TransferGetResponse> {
        let mut r = self.http_client.client.post("/transfer/get");
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for TransferGetRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}