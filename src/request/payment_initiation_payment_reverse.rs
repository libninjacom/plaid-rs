use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentReverseRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub amount: Option<PaymentAmountToRefund>,
    pub idempotency_key: String,
    pub payment_id: String,
    pub reference: String,
}
impl<'a> PaymentInitiationPaymentReverseRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationPaymentReverseResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/payment/reverse");
        if let Some(ref unwrapped) = self.amount {
            r = r.json(json!({ "amount" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "payment_id" : self.payment_id }));
        r = r.json(json!({ "reference" : self.reference }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn amount(mut self, amount: PaymentAmountToRefund) -> Self {
        self.amount = Some(amount);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentReverseRequest<'a> {
    type Output = crate::Result<PaymentInitiationPaymentReverseResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
