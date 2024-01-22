use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_liabilities_get`].

On request success, this will return a [`ProcessorLiabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorLiabilitiesGetRequest {
    pub processor_token: String,
}
impl ProcessorLiabilitiesGetRequest {}
impl FluentRequest<'_, ProcessorLiabilitiesGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorLiabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorLiabilitiesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/liabilities/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}