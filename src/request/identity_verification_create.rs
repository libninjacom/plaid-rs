use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::identity_verification_create`].

On request success, this will return a [`IdentityVerificationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationCreateRequest {
    pub client_user_id: Option<String>,
    pub gave_consent: bool,
    pub is_idempotent: Option<bool>,
    pub is_shareable: bool,
    pub template_id: String,
    pub user: Option<IdentityVerificationCreateRequestUser>,
}
impl IdentityVerificationCreateRequest {}
impl FluentRequest<'_, IdentityVerificationCreateRequest> {
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn is_idempotent(mut self, is_idempotent: bool) -> Self {
        self.params.is_idempotent = Some(is_idempotent);
        self
    }
    pub fn user(mut self, user: IdentityVerificationCreateRequestUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationCreateRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/identity_verification/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}