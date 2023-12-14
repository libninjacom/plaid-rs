use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_token_permissions_get`].

On request success, this will return a [`ProcessorTokenPermissionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenPermissionsGetRequest {
    pub processor_token: String,
}
impl ProcessorTokenPermissionsGetRequest {}
impl FluentRequest<'_, ProcessorTokenPermissionsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenPermissionsGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTokenPermissionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/token/permissions/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}