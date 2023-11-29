use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferSweepGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub sweep_id: String,
}
impl<'a> BankTransferSweepGetRequest<'a> {
    pub async fn send(self) -> crate::Result<BankTransferSweepGetResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/sweep/get");
        r = r.json(json!({ "sweep_id" : self.sweep_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferSweepGetRequest<'a> {
    type Output = crate::Result<BankTransferSweepGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
