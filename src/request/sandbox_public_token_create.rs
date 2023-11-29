use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxPublicTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub initial_products: Vec<String>,
    pub institution_id: String,
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
    pub user_token: Option<String>,
}
impl<'a> SandboxPublicTokenCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxPublicTokenCreateResponse> {
        let mut r = self.http_client.client.post("/sandbox/public_token/create");
        r = r.json(json!({ "initial_products" : self.initial_products }));
        r = r.json(json!({ "institution_id" : self.institution_id }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: SandboxPublicTokenCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for SandboxPublicTokenCreateRequest<'a> {
    type Output = crate::Result<SandboxPublicTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
