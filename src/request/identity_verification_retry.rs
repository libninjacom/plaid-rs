use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::identity_verification_retry`].

On request success, this will return a [`IdentityVerificationRetryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequest {
    pub client_user_id: String,
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
    pub strategy: String,
    pub template_id: String,
    pub user: Option<IdentityVerificationRequestUser>,
}
impl IdentityVerificationRetryRequest {}
impl FluentRequest<'_, IdentityVerificationRetryRequest> {
    pub fn steps(mut self, steps: IdentityVerificationRetryRequestStepsObject) -> Self {
        self.params.steps = Some(steps);
        self
    }
    pub fn user(mut self, user: IdentityVerificationRequestUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationRetryRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationRetryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/retry";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "client_user_id" : self.params.client_user_id }));
            if let Some(ref unwrapped) = self.params.steps {
                r = r.json(json!({ "steps" : unwrapped }));
            }
            r = r.json(json!({ "strategy" : self.params.strategy }));
            r = r.json(json!({ "template_id" : self.params.template_id }));
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}