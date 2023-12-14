use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportGetRequest {
    pub asset_report_token: Option<String>,
    pub fast_report: Option<bool>,
    pub include_insights: Option<bool>,
    pub options: Option<AssetReportGetRequestOptions>,
    pub user_token: Option<String>,
}
impl AssetReportGetRequest {}
impl FluentRequest<'_, AssetReportGetRequest> {
    pub fn asset_report_token(mut self, asset_report_token: &str) -> Self {
        self.params.asset_report_token = Some(asset_report_token.to_owned());
        self
    }
    pub fn fast_report(mut self, fast_report: bool) -> Self {
        self.params.fast_report = Some(fast_report);
        self
    }
    pub fn include_insights(mut self, include_insights: bool) -> Self {
        self.params.include_insights = Some(include_insights);
        self
    }
    pub fn options(mut self, options: AssetReportGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportGetRequest> {
    type Output = httpclient::InMemoryResult<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}