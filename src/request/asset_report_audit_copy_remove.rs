use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_remove`].

On request success, this will return a [`AssetReportAuditCopyRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveRequest {
    pub audit_copy_token: String,
}
impl AssetReportAuditCopyRemoveRequest {}
impl FluentRequest<'_, AssetReportAuditCopyRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyRemoveRequest> {
    type Output = httpclient::InMemoryResult<AssetReportAuditCopyRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/audit_copy/remove";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "audit_copy_token" : self.params.audit_copy_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}