use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_application_scopes_update`].

On request success, this will return a [`ItemApplicationScopesUpdateResponse`].*/
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
        Box::pin(async move {
            let url = "/item/application/scopes/update";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "application_id" : self.params.application_id }));
            r = r.json(json!({ "context" : self.params.context }));
            r = r.json(json!({ "scopes" : self.params.scopes }));
            if let Some(ref unwrapped) = self.params.state {
                r = r.json(json!({ "state" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}