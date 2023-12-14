use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/payment_initiation/payment/reverse";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}