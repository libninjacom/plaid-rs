use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListRequest {
    pub consent_id: Option<String>,
    pub count: Option<i64>,
    pub cursor: Option<chrono::DateTime<chrono::Utc>>,
}
impl PaymentInitiationPaymentListRequest {}
impl FluentRequest<'_, PaymentInitiationPaymentListRequest> {
    pub fn consent_id(mut self, consent_id: &str) -> Self {
        self.params.consent_id = Some(consent_id.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.cursor = Some(cursor);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentListRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/payment_initiation/payment/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}