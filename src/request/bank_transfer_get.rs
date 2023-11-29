use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferGetRequest<'a> {
    pub async fn send(self) -> crate::Result<BankTransferGetResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/get");
        r = r.json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferGetRequest<'a> {
    type Output = crate::Result<BankTransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
