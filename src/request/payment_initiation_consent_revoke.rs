use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_revoke`].

On request success, this will return a [`PaymentInitiationConsentRevokeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeRequest {
    pub consent_id: String,
}
impl PaymentInitiationConsentRevokeRequest {}
impl FluentRequest<'_, PaymentInitiationConsentRevokeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentRevokeRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationConsentRevokeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/revoke";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "consent_id" : self.params.consent_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}