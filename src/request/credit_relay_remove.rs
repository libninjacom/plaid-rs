use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditRelayRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub relay_token: String,
}
impl<'a> CreditRelayRemoveRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreditRelayRemoveResponse> {
        let mut r = self.http_client.client.post("/credit/relay/remove");
        r = r.json(json!({ "relay_token" : self.relay_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreditRelayRemoveRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditRelayRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
