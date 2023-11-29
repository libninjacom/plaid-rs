use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemApplicationListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
}
impl<'a> ItemApplicationListRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemApplicationListResponse> {
        let mut r = self.http_client.client.post("/item/application/list");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ItemApplicationListRequest<'a> {
    type Output = crate::Result<ItemApplicationListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
