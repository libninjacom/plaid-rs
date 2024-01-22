use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_token_permissions_set`].

On request success, this will return a [`ProcessorTokenPermissionsSetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenPermissionsSetRequest {
    pub processor_token: String,
    pub products: Vec<String>,
}
impl ProcessorTokenPermissionsSetRequest {}
impl FluentRequest<'_, ProcessorTokenPermissionsSetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenPermissionsSetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTokenPermissionsSetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/permissions/set";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = r.json(json!({ "products" : self.params.products }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}