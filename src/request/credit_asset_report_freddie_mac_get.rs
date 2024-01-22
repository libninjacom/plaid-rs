use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_asset_report_freddie_mac_get`].

On request success, this will return a [`AssetReportFreddieGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAssetReportFreddieMacGetRequest {
    pub audit_copy_token: String,
}
impl CreditAssetReportFreddieMacGetRequest {}
impl FluentRequest<'_, CreditAssetReportFreddieMacGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAssetReportFreddieMacGetRequest> {
    type Output = httpclient::InMemoryResult<AssetReportFreddieGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/asset_report/freddie_mac/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "audit_copy_token" : self.params.audit_copy_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}