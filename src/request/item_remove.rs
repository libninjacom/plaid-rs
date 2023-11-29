use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemRemoveRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemRemoveResponse> {
        let mut r = self.http_client.client.post("/item/remove");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for ItemRemoveRequest<'a> {
    type Output = crate::Result<ItemRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
