use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::identity_verification_list`].

On request success, this will return a [`IdentityVerificationListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationListRequest {
    pub client_user_id: String,
    pub cursor: Option<String>,
    pub template_id: String,
}
impl IdentityVerificationListRequest {}
impl FluentRequest<'_, IdentityVerificationListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationListRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/list";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "client_user_id" : self.params.client_user_id }));
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = r.json(json!({ "template_id" : self.params.template_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}