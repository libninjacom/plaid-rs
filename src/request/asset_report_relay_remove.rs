use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportRelayRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayRemoveRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<AssetReportRelayRemoveResponse> {
        let mut r = self.http_client.client.post("/asset_report/relay/remove");
        r = r.json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportRelayRemoveRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportRelayRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
