use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditRelayGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
}
impl<'a> CreditRelayGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<AssetReportGetResponse> {
        let mut r = self.http_client.client.post("/credit/relay/get");
        r = r.json(json!({ "relay_token" : self.relay_token }));
        r = r.json(json!({ "report_type" : self.report_type }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreditRelayGetRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}