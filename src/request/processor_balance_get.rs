use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_balance_get`].

On request success, this will return a [`ProcessorBalanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequest {
    pub options: Option<ProcessorBalanceGetRequestOptions>,
    pub processor_token: String,
}
impl ProcessorBalanceGetRequest {}
impl FluentRequest<'_, ProcessorBalanceGetRequest> {
    pub fn options(mut self, options: ProcessorBalanceGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/balance/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}