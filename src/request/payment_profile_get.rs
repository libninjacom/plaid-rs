use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_profile_get`].

On request success, this will return a [`PaymentProfileGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileGetRequest {
    pub payment_profile_token: String,
}
impl PaymentProfileGetRequest {}
impl FluentRequest<'_, PaymentProfileGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileGetRequest> {
    type Output = httpclient::InMemoryResult<PaymentProfileGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_profile/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "payment_profile_token" : self.params.payment_profile_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}