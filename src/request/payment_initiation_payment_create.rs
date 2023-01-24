use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub amount: PaymentAmount,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
    pub options: Option<ExternalPaymentOptions>,
}
impl<'a> PaymentInitiationPaymentCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationPaymentCreateResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/payment/create");
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = r.json(json!({ "reference" : self.reference }));
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.schedule {
            r = r.json(json!({ "schedule" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn schedule(mut self, schedule: ExternalPaymentScheduleRequest) -> Self {
        self.schedule = Some(schedule);
        self
    }
    pub fn options(mut self, options: ExternalPaymentOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
