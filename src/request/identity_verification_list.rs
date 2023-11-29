use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityVerificationListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_user_id: String,
    pub cursor: Option<String>,
    pub template_id: String,
}
impl<'a> IdentityVerificationListRequest<'a> {
    pub async fn send(self) -> crate::Result<IdentityVerificationListResponse> {
        let mut r = self.http_client.client.post("/identity_verification/list");
        r = r.json(json!({ "client_user_id" : self.client_user_id }));
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = r.json(json!({ "template_id" : self.template_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for IdentityVerificationListRequest<'a> {
    type Output = crate::Result<IdentityVerificationListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
