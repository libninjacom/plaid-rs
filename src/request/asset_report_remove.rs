use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
}
impl<'a> AssetReportRemoveRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportRemoveResponse> {
        let mut r = self.http_client.client.post("/asset_report/remove");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportRemoveRequest<'a> {
    type Output = crate::Result<AssetReportRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
