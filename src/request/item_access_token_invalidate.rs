use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemAccessTokenInvalidateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemAccessTokenInvalidateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ItemAccessTokenInvalidateResponse> {
        let mut r = self.http_client.client.post("/item/access_token/invalidate");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for ItemAccessTokenInvalidateRequest<'a> {
    type Output = httpclient::InMemoryResult<ItemAccessTokenInvalidateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}