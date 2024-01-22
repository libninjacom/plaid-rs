use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::identity_verification_autofill_create`].

On request success, this will return a [`IdentityVerificationAutofillCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationAutofillCreateRequest {
    pub identity_verification_id: String,
}
impl IdentityVerificationAutofillCreateRequest {}
impl FluentRequest<'_, IdentityVerificationAutofillCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationAutofillCreateRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationAutofillCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/autofill/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "identity_verification_id" : self.params
                        .identity_verification_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}