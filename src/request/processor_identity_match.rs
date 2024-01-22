use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_identity_match`].

On request success, this will return a [`ProcessorIdentityMatchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityMatchRequest {
    pub processor_token: String,
    pub user: Option<IdentityMatchUser>,
}
impl ProcessorIdentityMatchRequest {}
impl FluentRequest<'_, ProcessorIdentityMatchRequest> {
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorIdentityMatchRequest> {
    type Output = httpclient::InMemoryResult<ProcessorIdentityMatchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/identity/match";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}