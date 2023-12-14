use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_identity_get`].

On request success, this will return a [`ProcessorIdentityGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    pub processor_token: String,
}
impl ProcessorIdentityGetRequest {}
impl FluentRequest<'_, ProcessorIdentityGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorIdentityGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorIdentityGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/identity/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}