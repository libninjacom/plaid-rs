use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_auth_get`].

On request success, this will return a [`ProcessorAuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorAuthGetRequest {
    pub processor_token: String,
}
impl ProcessorAuthGetRequest {}
impl FluentRequest<'_, ProcessorAuthGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorAuthGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorAuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/auth/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}