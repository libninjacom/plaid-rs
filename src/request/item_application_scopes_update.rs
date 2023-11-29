use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemApplicationScopesUpdateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub application_id: String,
    pub context: String,
    pub scopes: Scopes,
    pub state: Option<String>,
}
impl<'a> ItemApplicationScopesUpdateRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemApplicationScopesUpdateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/item/application/scopes/update");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "application_id" : self.application_id }));
        r = r.json(json!({ "context" : self.context }));
        r = r.json(json!({ "scopes" : self.scopes }));
        if let Some(ref unwrapped) = self.state {
            r = r.json(json!({ "state" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
}
pub struct ItemApplicationScopesUpdateRequired<'a> {
    pub access_token: &'a str,
    pub application_id: &'a str,
    pub context: &'a str,
    pub scopes: Scopes,
}
impl<'a> ItemApplicationScopesUpdateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for ItemApplicationScopesUpdateRequest<'a> {
    type Output = crate::Result<ItemApplicationScopesUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
