use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityVerificationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub is_shareable: bool,
    pub template_id: String,
    pub gave_consent: bool,
    pub user: IdentityVerificationRequestUser,
    pub is_idempotent: Option<bool>,
}
impl<'a> IdentityVerificationCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<IdentityVerificationCreateResponse> {
        let mut r = self.http_client.client.post("/identity_verification/create");
        r = r.json(json!({ "is_shareable" : self.is_shareable }));
        r = r.json(json!({ "template_id" : self.template_id }));
        r = r.json(json!({ "gave_consent" : self.gave_consent }));
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.is_idempotent {
            r = r.json(json!({ "is_idempotent" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn is_idempotent(mut self, is_idempotent: bool) -> Self {
        self.is_idempotent = Some(is_idempotent);
        self
    }
}
pub struct IdentityVerificationCreateRequired<'a> {
    pub is_shareable: bool,
    pub template_id: &'a str,
    pub gave_consent: bool,
    pub user: IdentityVerificationRequestUser,
}
impl<'a> IdentityVerificationCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for IdentityVerificationCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<IdentityVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}