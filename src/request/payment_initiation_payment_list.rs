use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_payment_list`].

On request success, this will return a [`PaymentInitiationPaymentListResponse`].*/
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
        Box::pin(async move {
            let url = "/payment_initiation/payment/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.consent_id {
                r = r.json(json!({ "consent_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}