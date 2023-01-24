use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportRelayCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> AssetReportRelayCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<AssetReportRelayCreateResponse> {
        let mut r = self.http_client.client.post("/asset_report/relay/create");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r.json(json!({ "secondary_client_id" : self.secondary_client_id }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.json(json!({ "webhook" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportRelayCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportRelayCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
