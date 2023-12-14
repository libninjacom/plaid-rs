use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_signal_prepare`].

On request success, this will return a [`ProcessorSignalPrepareResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalPrepareRequest {
    pub processor_token: String,
}
impl ProcessorSignalPrepareRequest {}
impl FluentRequest<'_, ProcessorSignalPrepareRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorSignalPrepareRequest> {
    type Output = httpclient::InMemoryResult<ProcessorSignalPrepareResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/signal/prepare";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}