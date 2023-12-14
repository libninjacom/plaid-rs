use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl AssetReportRefreshRequest {}
impl FluentRequest<'_, AssetReportRefreshRequest> {
    pub fn days_requested(mut self, days_requested: i64) -> Self {
        self.params.days_requested = Some(days_requested);
        self
    }
    pub fn options(mut self, options: AssetReportRefreshRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportRefreshRequest> {
    type Output = httpclient::InMemoryResult<AssetReportRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/refresh";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}