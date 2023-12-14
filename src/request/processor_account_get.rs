use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_account_get`].

On request success, this will return a [`ProcessorAccountGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorAccountGetRequest {
    pub processor_token: String,
}
impl ProcessorAccountGetRequest {}
impl FluentRequest<'_, ProcessorAccountGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorAccountGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorAccountGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/account/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}