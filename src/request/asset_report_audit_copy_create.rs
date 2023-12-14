use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_create`].

On request success, this will return a [`AssetReportAuditCopyCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateRequest {
    pub asset_report_token: String,
    pub auditor_id: Option<String>,
}
impl AssetReportAuditCopyCreateRequest {}
impl FluentRequest<'_, AssetReportAuditCopyCreateRequest> {
    pub fn auditor_id(mut self, auditor_id: &str) -> Self {
        self.params.auditor_id = Some(auditor_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyCreateRequest> {
    type Output = httpclient::InMemoryResult<AssetReportAuditCopyCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/audit_copy/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}