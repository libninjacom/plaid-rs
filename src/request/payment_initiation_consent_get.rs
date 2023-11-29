use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationConsentGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentGetRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationConsentGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/consent/get");
        r = r.json(json!({ "consent_id" : self.consent_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationConsentGetRequest<'a> {
    type Output = crate::Result<PaymentInitiationConsentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
