use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkDeliveryCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub link_token: String,
    pub delivery_method: String,
    pub delivery_destination: String,
}
impl<'a> LinkDeliveryCreateRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<LinkDeliveryCreateResponse> {
        let mut r = self.http_client.client.post("/link_delivery/create");
        r = r.json(json!({ "link_token" : self.link_token }));
        r = r.json(json!({ "delivery_method" : self.delivery_method }));
        r = r.json(json!({ "delivery_destination" : self.delivery_destination }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for LinkDeliveryCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<LinkDeliveryCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}