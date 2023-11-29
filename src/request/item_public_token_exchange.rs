use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemPublicTokenExchangeRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub public_token: String,
}
impl<'a> ItemPublicTokenExchangeRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemPublicTokenExchangeResponse> {
        let mut r = self.http_client.client.post("/item/public_token/exchange");
        r = r.json(json!({ "public_token" : self.public_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for ItemPublicTokenExchangeRequest<'a> {
    type Output = crate::Result<ItemPublicTokenExchangeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
