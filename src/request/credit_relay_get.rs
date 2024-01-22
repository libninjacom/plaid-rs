use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_relay_get`].

On request success, this will return a [`AssetReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayGetRequest {
    pub relay_token: String,
    pub report_type: String,
}
impl CreditRelayGetRequest {}
impl FluentRequest<'_, CreditRelayGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayGetRequest> {
    type Output = httpclient::InMemoryResult<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "relay_token" : self.params.relay_token }));
            r = r.json(json!({ "report_type" : self.params.report_type }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}