use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_processor_token_create`].

On request success, this will return a [`SandboxProcessorTokenCreateResponse`].*/
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
        Box::pin(async move {
            let url = "/sandbox/processor_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "institution_id" : self.params.institution_id }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}