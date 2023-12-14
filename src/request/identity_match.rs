use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMatchRequest {
    pub access_token: String,
    pub options: Option<IdentityMatchRequestOptions>,
    pub user: Option<IdentityMatchUser>,
}
impl IdentityMatchRequest {}
impl FluentRequest<'_, IdentityMatchRequest> {
    pub fn options(mut self, options: IdentityMatchRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IdentityMatchRequest> {
    type Output = httpclient::InMemoryResult<IdentityMatchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/identity/match";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}