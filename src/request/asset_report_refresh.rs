use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportRefreshRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl<'a> AssetReportRefreshRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportRefreshResponse> {
        let mut r = self.http_client.client.post("/asset_report/refresh");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.days_requested {
            r = r.json(json!({ "days_requested" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn days_requested(mut self, days_requested: i64) -> Self {
        self.days_requested = Some(days_requested);
        self
    }
    pub fn options(mut self, options: AssetReportRefreshRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportRefreshRequest<'a> {
    type Output = crate::Result<AssetReportRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
