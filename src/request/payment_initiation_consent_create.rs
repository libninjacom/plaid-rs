use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationConsentCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
}
impl<'a> PaymentInitiationConsentCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationConsentCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/consent/create");
        r = r.json(json!({ "constraints" : self.constraints }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = r.json(json!({ "reference" : self.reference }));
        r = r.json(json!({ "scopes" : self.scopes }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: ExternalPaymentInitiationConsentOptions) -> Self {
        self.options = Some(options);
        self
    }
}
pub struct PaymentInitiationConsentCreateRequired<'a> {
    pub constraints: PaymentInitiationConsentConstraints,
    pub recipient_id: &'a str,
    pub reference: &'a str,
    pub scopes: &'a [&'a str],
}
impl<'a> PaymentInitiationConsentCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for PaymentInitiationConsentCreateRequest<'a> {
    type Output = crate::Result<PaymentInitiationConsentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
