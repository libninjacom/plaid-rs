use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityVerificationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub gave_consent: bool,
    pub is_idempotent: Option<bool>,
    pub is_shareable: bool,
    pub template_id: String,
    pub user: IdentityVerificationRequestUser,
}
impl<'a> IdentityVerificationCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<IdentityVerificationCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/identity_verification/create");
        r = r.json(json!({ "gave_consent" : self.gave_consent }));
        if let Some(ref unwrapped) = self.is_idempotent {
            r = r.json(json!({ "is_idempotent" : unwrapped }));
        }
        r = r.json(json!({ "is_shareable" : self.is_shareable }));
        r = r.json(json!({ "template_id" : self.template_id }));
        r = r.json(json!({ "user" : self.user }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn is_idempotent(mut self, is_idempotent: bool) -> Self {
        self.is_idempotent = Some(is_idempotent);
        self
    }
}
pub struct IdentityVerificationCreateRequired<'a> {
    pub gave_consent: bool,
    pub is_shareable: bool,
    pub template_id: &'a str,
    pub user: IdentityVerificationRequestUser,
}
impl<'a> IdentityVerificationCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for IdentityVerificationCreateRequest<'a> {
    type Output = crate::Result<IdentityVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
