use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityVerificationRetryRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_user_id: String,
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
    pub strategy: String,
    pub template_id: String,
}
impl<'a> IdentityVerificationRetryRequest<'a> {
    pub async fn send(self) -> crate::Result<IdentityVerificationRetryResponse> {
        let mut r = self.http_client.client.post("/identity_verification/retry");
        r = r.json(json!({ "client_user_id" : self.client_user_id }));
        if let Some(ref unwrapped) = self.steps {
            r = r.json(json!({ "steps" : unwrapped }));
        }
        r = r.json(json!({ "strategy" : self.strategy }));
        r = r.json(json!({ "template_id" : self.template_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn steps(mut self, steps: IdentityVerificationRetryRequestStepsObject) -> Self {
        self.steps = Some(steps);
        self
    }
}
impl<'a> ::std::future::IntoFuture for IdentityVerificationRetryRequest<'a> {
    type Output = crate::Result<IdentityVerificationRetryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
