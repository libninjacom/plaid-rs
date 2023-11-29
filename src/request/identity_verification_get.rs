use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IdentityVerificationGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub identity_verification_id: String,
}
impl<'a> IdentityVerificationGetRequest<'a> {
    pub async fn send(self) -> crate::Result<IdentityVerificationGetResponse> {
        let mut r = self.http_client.client.post("/identity_verification/get");
        r = r.json(json!({ "identity_verification_id" : self.identity_verification_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for IdentityVerificationGetRequest<'a> {
    type Output = crate::Result<IdentityVerificationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
