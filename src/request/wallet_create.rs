use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub iso_currency_code: String,
}
impl<'a> WalletCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<WalletCreateResponse> {
        let mut r = self.http_client.client.post("/wallet/create");
        r = r.json(json!({ "iso_currency_code" : self.iso_currency_code }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for WalletCreateRequest<'a> {
    type Output = crate::Result<WalletCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
