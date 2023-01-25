use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorAuthGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub processor_token: String,
}
impl<'a> ProcessorAuthGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ProcessorAuthGetResponse> {
        let mut r = self.http_client.client.post("/processor/auth/get");
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for ProcessorAuthGetRequest<'a> {
    type Output = httpclient::InMemoryResult<ProcessorAuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}