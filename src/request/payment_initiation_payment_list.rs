use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub consent_id: Option<String>,
    pub count: Option<i64>,
    pub cursor: Option<chrono::DateTime<chrono::Utc>>,
}
impl<'a> PaymentInitiationPaymentListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationPaymentListResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/payment/list");
        if let Some(ref unwrapped) = self.consent_id {
            r = r.json(json!({ "consent_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn consent_id(mut self, consent_id: &str) -> Self {
        self.consent_id = Some(consent_id.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: chrono::DateTime<chrono::Utc>) -> Self {
        self.cursor = Some(cursor);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentListRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}