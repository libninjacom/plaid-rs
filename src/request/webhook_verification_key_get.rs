use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::webhook_verification_key_get`].

On request success, this will return a [`WebhookVerificationKeyGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    pub key_id: String,
}
impl WebhookVerificationKeyGetRequest {}
impl FluentRequest<'_, WebhookVerificationKeyGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WebhookVerificationKeyGetRequest> {
    type Output = httpclient::InMemoryResult<WebhookVerificationKeyGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/webhook_verification_key/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "key_id" : self.params.key_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}