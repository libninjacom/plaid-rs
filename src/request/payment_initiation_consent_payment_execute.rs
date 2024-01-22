use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_payment_execute`].

On request success, this will return a [`PaymentInitiationConsentPaymentExecuteResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteRequest {
    pub amount: PaymentAmount,
    pub consent_id: String,
    pub idempotency_key: String,
}
impl PaymentInitiationConsentPaymentExecuteRequest {}
impl FluentRequest<'_, PaymentInitiationConsentPaymentExecuteRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentPaymentExecuteRequest> {
    type Output = httpclient::InMemoryResult<
        PaymentInitiationConsentPaymentExecuteResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/payment/execute";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "amount" : self.params.amount }));
            r = r.json(json!({ "consent_id" : self.params.consent_id }));
            r = r.json(json!({ "idempotency_key" : self.params.idempotency_key }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}