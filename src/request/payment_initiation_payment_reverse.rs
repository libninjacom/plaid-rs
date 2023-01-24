use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentReverseRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub payment_id: String,
    pub idempotency_key: String,
    pub reference: String,
}
impl<'a> PaymentInitiationPaymentReverseRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationPaymentReverseResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/payment/reverse");
        r = r.json(json!({ "payment_id" : self.payment_id }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "reference" : self.reference }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentReverseRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentReverseResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
