use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WebhookVerificationKeyGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub key_id: String,
}
impl<'a> WebhookVerificationKeyGetRequest<'a> {
    pub async fn send(self) -> crate::Result<WebhookVerificationKeyGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/webhook_verification_key/get");
        r = r.json(json!({ "key_id" : self.key_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for WebhookVerificationKeyGetRequest<'a> {
    type Output = crate::Result<WebhookVerificationKeyGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
