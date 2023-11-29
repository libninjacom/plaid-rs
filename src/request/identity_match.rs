use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityMatchRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<IdentityMatchRequestOptions>,
    pub user: Option<IdentityMatchUser>,
}
impl<'a> IdentityMatchRequest<'a> {
    pub async fn send(self) -> crate::Result<IdentityMatchResponse> {
        let mut r = self.http_client.client.post("/identity/match");
        r = r.json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: IdentityMatchRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for IdentityMatchRequest<'a> {
    type Output = crate::Result<IdentityMatchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
