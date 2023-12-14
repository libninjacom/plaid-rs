use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/processor/identity/match";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}