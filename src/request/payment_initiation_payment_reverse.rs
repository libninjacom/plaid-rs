use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_payment_reverse`].

On request success, this will return a [`PaymentInitiationPaymentReverseResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    pub amount: Option<PaymentAmountToRefund>,
    pub idempotency_key: String,
    pub payment_id: String,
    pub reference: String,
}
impl PaymentInitiationPaymentReverseRequest {}
impl FluentRequest<'_, PaymentInitiationPaymentReverseRequest> {
    pub fn amount(mut self, amount: PaymentAmountToRefund) -> Self {
        self.params.amount = Some(amount);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentReverseRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentReverseResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/reverse";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.amount {
                r = r.json(json!({ "amount" : unwrapped }));
            }
            r = r.json(json!({ "idempotency_key" : self.params.idempotency_key }));
            r = r.json(json!({ "payment_id" : self.params.payment_id }));
            r = r.json(json!({ "reference" : self.params.reference }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}