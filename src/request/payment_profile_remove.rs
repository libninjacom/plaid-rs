use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_profile_remove`].

On request success, this will return a [`PaymentProfileRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileRemoveRequest {
    pub payment_profile_token: String,
}
impl PaymentProfileRemoveRequest {}
impl FluentRequest<'_, PaymentProfileRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileRemoveRequest> {
    type Output = httpclient::InMemoryResult<PaymentProfileRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/payment_profile/remove";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}