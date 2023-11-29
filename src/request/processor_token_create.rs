use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl<'a> ProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<ProcessorTokenCreateResponse> {
        let mut r = self.http_client.client.post("/processor/token/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_id" : self.account_id }));
        r = r.json(json!({ "processor" : self.processor }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for ProcessorTokenCreateRequest<'a> {
    type Output = crate::Result<ProcessorTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
