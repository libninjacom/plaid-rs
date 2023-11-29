use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkDeliveryGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub link_delivery_session_id: String,
}
impl<'a> LinkDeliveryGetRequest<'a> {
    pub async fn send(self) -> crate::Result<LinkDeliveryGetResponse> {
        let mut r = self.http_client.client.post("/link_delivery/get");
        r = r.json(json!({ "link_delivery_session_id" : self.link_delivery_session_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for LinkDeliveryGetRequest<'a> {
    type Output = crate::Result<LinkDeliveryGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
