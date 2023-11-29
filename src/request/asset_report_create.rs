use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_tokens: Option<Vec<String>>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
    pub report_type: Option<String>,
    pub user_token: Option<String>,
}
impl<'a> AssetReportCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportCreateResponse> {
        let mut r = self.http_client.client.post("/asset_report/create");
        if let Some(ref unwrapped) = self.access_tokens {
            r = r.json(json!({ "access_tokens" : unwrapped }));
        }
        r = r.json(json!({ "days_requested" : self.days_requested }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.report_type {
            r = r.json(json!({ "report_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.access_tokens = Some(
            access_tokens
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn options(mut self, options: AssetReportCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn report_type(mut self, report_type: &str) -> Self {
        self.report_type = Some(report_type.to_owned());
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportCreateRequest<'a> {
    type Output = crate::Result<AssetReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
