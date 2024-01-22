use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_create`].

On request success, this will return a [`AssetReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    pub access_tokens: Option<Vec<String>>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
}
impl AssetReportCreateRequest {}
impl FluentRequest<'_, AssetReportCreateRequest> {
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn options(mut self, options: AssetReportCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportCreateRequest> {
    type Output = httpclient::InMemoryResult<AssetReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(json!({ "access_tokens" : unwrapped }));
            }
            r = r.json(json!({ "days_requested" : self.params.days_requested }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}