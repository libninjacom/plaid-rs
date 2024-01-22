use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_relay_remove`].

On request success, this will return a [`CreditRelayRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayRemoveRequest {
    pub relay_token: String,
}
impl CreditRelayRemoveRequest {}
impl FluentRequest<'_, CreditRelayRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayRemoveRequest> {
    type Output = httpclient::InMemoryResult<CreditRelayRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/remove";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "relay_token" : self.params.relay_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}