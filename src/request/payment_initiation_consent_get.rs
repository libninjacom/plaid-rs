use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_get`].

On request success, this will return a [`PaymentInitiationConsentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetRequest {
    pub consent_id: String,
}
impl PaymentInitiationConsentGetRequest {}
impl FluentRequest<'_, PaymentInitiationConsentGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentGetRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationConsentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "consent_id" : self.params.consent_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}