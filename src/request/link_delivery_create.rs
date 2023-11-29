use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkDeliveryCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub link_token: String,
    pub options: Option<LinkDeliveryOptions>,
}
impl<'a> LinkDeliveryCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<LinkDeliveryCreateResponse> {
        let mut r = self.http_client.client.post("/link_delivery/create");
        r = r.json(json!({ "link_token" : self.link_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: LinkDeliveryOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for LinkDeliveryCreateRequest<'a> {
    type Output = crate::Result<LinkDeliveryCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
