use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::signal_prepare`].

On request success, this will return a [`SignalPrepareResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalPrepareRequest {
    pub access_token: String,
}
impl SignalPrepareRequest {}
impl FluentRequest<'_, SignalPrepareRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalPrepareRequest> {
    type Output = httpclient::InMemoryResult<SignalPrepareResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/signal/prepare";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}