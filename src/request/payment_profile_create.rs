use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_profile_create`].

On request success, this will return a [`PaymentProfileCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileCreateRequest {}
impl PaymentProfileCreateRequest {}
impl FluentRequest<'_, PaymentProfileCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileCreateRequest> {
    type Output = httpclient::InMemoryResult<PaymentProfileCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_profile/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}