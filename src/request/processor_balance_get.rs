use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorBalanceGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<ProcessorBalanceGetRequestOptions>,
    pub processor_token: String,
}
impl<'a> ProcessorBalanceGetRequest<'a> {
    pub async fn send(self) -> crate::Result<ProcessorBalanceGetResponse> {
        let mut r = self.http_client.client.post("/processor/balance/get");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: ProcessorBalanceGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ProcessorBalanceGetRequest<'a> {
    type Output = crate::Result<ProcessorBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
