use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_tokens: Vec<String>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
}
impl<'a> AssetReportCreateRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<AssetReportCreateResponse> {
        let mut r = self.http_client.client.post("/asset_report/create");
        r = r.json(json!({ "access_tokens" : self.access_tokens }));
        r = r.json(json!({ "days_requested" : self.days_requested }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: AssetReportCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
