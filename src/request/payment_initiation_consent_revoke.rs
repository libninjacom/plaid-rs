use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationConsentRevokeRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentRevokeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationConsentRevokeResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/consent/revoke");
        r = r.json(json!({ "consent_id" : self.consent_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationConsentRevokeRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationConsentRevokeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
