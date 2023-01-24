use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportRelayGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<AssetReportGetResponse> {
        let mut r = self.http_client.client.post("/asset_report/relay/get");
        r = r.json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportRelayGetRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
