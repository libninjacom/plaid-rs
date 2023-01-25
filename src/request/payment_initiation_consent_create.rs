use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationConsentCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
}
impl<'a> PaymentInitiationConsentCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationConsentCreateResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/consent/create");
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = r.json(json!({ "reference" : self.reference }));
        r = r.json(json!({ "scopes" : self.scopes }));
        r = r.json(json!({ "constraints" : self.constraints }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: ExternalPaymentInitiationConsentOptions) -> Self {
        self.options = Some(options);
        self
    }
}
pub struct PaymentInitiationConsentCreateRequired<'a> {
    pub recipient_id: &'a str,
    pub reference: &'a str,
    pub scopes: &'a [&'a str],
    pub constraints: PaymentInitiationConsentConstraints,
}
impl<'a> PaymentInitiationConsentCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for PaymentInitiationConsentCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationConsentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}