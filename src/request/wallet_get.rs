use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub wallet_id: String,
}
impl<'a> WalletGetRequest<'a> {
    pub async fn send(self) -> crate::Result<WalletGetResponse> {
        let mut r = self.http_client.client.post("/wallet/get");
        r = r.json(json!({ "wallet_id" : self.wallet_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for WalletGetRequest<'a> {
    type Output = crate::Result<WalletGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
