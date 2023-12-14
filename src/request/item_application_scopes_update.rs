use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    pub access_token: String,
    pub application_id: String,
    pub context: String,
    pub scopes: Scopes,
    pub state: Option<String>,
}
impl ItemApplicationScopesUpdateRequest {}
pub struct ItemApplicationScopesUpdateRequired<'a> {
    pub access_token: &'a str,
    pub application_id: &'a str,
    pub context: &'a str,
    pub scopes: Scopes,
}
impl<'a> ItemApplicationScopesUpdateRequired<'a> {}
impl FluentRequest<'_, ItemApplicationScopesUpdateRequest> {
    pub fn state(mut self, state: &str) -> Self {
        self.params.state = Some(state.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemApplicationScopesUpdateRequest> {
    type Output = httpclient::InMemoryResult<ItemApplicationScopesUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/item/application/scopes/update";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}