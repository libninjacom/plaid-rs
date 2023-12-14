use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_filter`].

On request success, this will return a [`AssetReportFilterResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    pub account_ids_to_exclude: Vec<String>,
    pub asset_report_token: String,
}
impl AssetReportFilterRequest {}
impl FluentRequest<'_, AssetReportFilterRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportFilterRequest> {
    type Output = httpclient::InMemoryResult<AssetReportFilterResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/filter";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}