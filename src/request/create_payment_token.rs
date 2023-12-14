use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::create_payment_token`].

On request success, this will return a [`PaymentInitiationPaymentTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentTokenRequest {
    pub payment_id: String,
}
impl CreatePaymentTokenRequest {}
impl FluentRequest<'_, CreatePaymentTokenRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreatePaymentTokenRequest> {
    type Output = httpclient::InMemoryResult<
        PaymentInitiationPaymentTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/payment_initiation/payment/token/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}