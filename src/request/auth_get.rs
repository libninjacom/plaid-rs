use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::auth_get`].

On request success, this will return a [`AuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthGetRequest {
    pub access_token: String,
    pub options: Option<AuthGetRequestOptions>,
}
impl AuthGetRequest {}
impl FluentRequest<'_, AuthGetRequest> {
    pub fn options(mut self, options: AuthGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AuthGetRequest> {
    type Output = httpclient::InMemoryResult<AuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/auth/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}