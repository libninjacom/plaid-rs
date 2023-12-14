use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::link_oauth_correlation_id_exchange`].

On request success, this will return a [`LinkOAuthCorrelationIdExchangeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkOauthCorrelationIdExchangeRequest {
    pub link_correlation_id: String,
}
impl LinkOauthCorrelationIdExchangeRequest {}
impl FluentRequest<'_, LinkOauthCorrelationIdExchangeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, LinkOauthCorrelationIdExchangeRequest> {
    type Output = httpclient::InMemoryResult<LinkOAuthCorrelationIdExchangeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/link/oauth/correlation_id/exchange";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}