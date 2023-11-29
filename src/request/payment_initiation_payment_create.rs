use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub amount: PaymentAmount,
    pub options: Option<ExternalPaymentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
}
impl<'a> PaymentInitiationPaymentCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationPaymentCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/payment/create");
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = r.json(json!({ "reference" : self.reference }));
        if let Some(ref unwrapped) = self.schedule {
            r = r.json(json!({ "schedule" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: ExternalPaymentOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn schedule(mut self, schedule: ExternalPaymentScheduleRequest) -> Self {
        self.schedule = Some(schedule);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentCreateRequest<'a> {
    type Output = crate::Result<PaymentInitiationPaymentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
