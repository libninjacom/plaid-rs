use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::link_delivery_get`].

On request success, this will return a [`LinkDeliveryGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryGetRequest {
    pub link_delivery_session_id: String,
}
impl LinkDeliveryGetRequest {}
impl FluentRequest<'_, LinkDeliveryGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkDeliveryGetRequest> {
    type Output = httpclient::InMemoryResult<LinkDeliveryGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link_delivery/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "link_delivery_session_id" : self.params
                        .link_delivery_session_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}