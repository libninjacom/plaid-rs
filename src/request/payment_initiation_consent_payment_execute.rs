use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub amount: PaymentAmount,
    pub consent_id: String,
    pub idempotency_key: String,
}
impl<'a> PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationConsentPaymentExecuteResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/consent/payment/execute");
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "consent_id" : self.consent_id }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationConsentPaymentExecuteRequest<'a> {
    type Output = crate::Result<PaymentInitiationConsentPaymentExecuteResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
