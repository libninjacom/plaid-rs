use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_create`].

On request success, this will return a [`PaymentInitiationConsentCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
}
impl PaymentInitiationConsentCreateRequest {}
pub struct PaymentInitiationConsentCreateRequired<'a> {
    pub constraints: PaymentInitiationConsentConstraints,
    pub recipient_id: &'a str,
    pub reference: &'a str,
    pub scopes: &'a [&'a str],
}
impl<'a> PaymentInitiationConsentCreateRequired<'a> {}
impl FluentRequest<'_, PaymentInitiationConsentCreateRequest> {
    pub fn options(mut self, options: ExternalPaymentInitiationConsentOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentCreateRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationConsentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "constraints" : self.params.constraints }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "recipient_id" : self.params.recipient_id }));
            r = r.json(json!({ "reference" : self.params.reference }));
            r = r.json(json!({ "scopes" : self.params.scopes }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}