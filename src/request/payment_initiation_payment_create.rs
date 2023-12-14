use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    pub amount: PaymentAmount,
    pub options: Option<ExternalPaymentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
}
impl PaymentInitiationPaymentCreateRequest {}
impl FluentRequest<'_, PaymentInitiationPaymentCreateRequest> {
    pub fn options(mut self, options: ExternalPaymentOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn schedule(mut self, schedule: ExternalPaymentScheduleRequest) -> Self {
        self.params.schedule = Some(schedule);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentCreateRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/payment_initiation/payment/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}