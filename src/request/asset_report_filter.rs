use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportFilterRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub account_ids_to_exclude: Vec<String>,
}
impl<'a> AssetReportFilterRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<AssetReportFilterResponse> {
        let mut r = self.http_client.client.post("/asset_report/filter");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r.json(json!({ "account_ids_to_exclude" : self.account_ids_to_exclude }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportFilterRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportFilterResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}