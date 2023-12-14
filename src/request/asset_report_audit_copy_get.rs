use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_get`].

On request success, this will return a [`AssetReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyGetRequest {
    pub audit_copy_token: String,
}
impl AssetReportAuditCopyGetRequest {}
impl FluentRequest<'_, AssetReportAuditCopyGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyGetRequest> {
    type Output = httpclient::InMemoryResult<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/audit_copy/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}