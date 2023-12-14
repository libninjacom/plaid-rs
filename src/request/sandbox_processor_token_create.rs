use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequest {
    pub institution_id: String,
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
impl SandboxProcessorTokenCreateRequest {}
impl FluentRequest<'_, SandboxProcessorTokenCreateRequest> {
    pub fn options(
        mut self,
        options: SandboxProcessorTokenCreateRequestOptions,
    ) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxProcessorTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<SandboxProcessorTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/processor_token/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}