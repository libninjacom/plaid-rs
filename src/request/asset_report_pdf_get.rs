use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportPdfGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub options: Option<AssetReportPdfGetRequestOptions>,
}
impl<'a> AssetReportPdfGetRequest<'a> {
    pub async fn send(self) -> crate::Result<()> {
        let mut r = self.http_client.client.post("/asset_report/pdf/get");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: AssetReportPdfGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportPdfGetRequest<'a> {
    type Output = crate::Result<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
