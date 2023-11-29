use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub fast_report: Option<bool>,
    pub include_insights: Option<bool>,
    pub options: Option<AssetReportGetRequestOptions>,
}
impl<'a> AssetReportGetRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportGetResponse> {
        let mut r = self.http_client.client.post("/asset_report/get");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.fast_report {
            r = r.json(json!({ "fast_report" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_insights {
            r = r.json(json!({ "include_insights" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn fast_report(mut self, fast_report: bool) -> Self {
        self.fast_report = Some(fast_report);
        self
    }
    pub fn include_insights(mut self, include_insights: bool) -> Self {
        self.include_insights = Some(include_insights);
        self
    }
    pub fn options(mut self, options: AssetReportGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportGetRequest<'a> {
    type Output = crate::Result<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
