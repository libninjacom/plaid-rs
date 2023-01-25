use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkOauthCorrelationIdExchangeRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub link_correlation_id: String,
}
impl<'a> LinkOauthCorrelationIdExchangeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<LinkOAuthCorrelationIdExchangeResponse> {
        let mut r = self.http_client.client.post("/link/oauth/correlation_id/exchange");
        r = r.json(json!({ "link_correlation_id" : self.link_correlation_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for LinkOauthCorrelationIdExchangeRequest<'a> {
    type Output = httpclient::InMemoryResult<LinkOAuthCorrelationIdExchangeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}