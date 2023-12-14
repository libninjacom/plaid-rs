use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::link_delivery_create`].

On request success, this will return a [`LinkDeliveryCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryCreateRequest {
    pub link_token: String,
    pub options: Option<LinkDeliveryOptions>,
}
impl LinkDeliveryCreateRequest {}
impl FluentRequest<'_, LinkDeliveryCreateRequest> {
    pub fn options(mut self, options: LinkDeliveryOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkDeliveryCreateRequest> {
    type Output = httpclient::InMemoryResult<LinkDeliveryCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/link_delivery/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}