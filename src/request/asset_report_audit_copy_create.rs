use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportAuditCopyCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub auditor_id: String,
}
impl<'a> AssetReportAuditCopyCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<AssetReportAuditCopyCreateResponse> {
        let mut r = self.http_client.client.post("/asset_report/audit_copy/create");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r.json(json!({ "auditor_id" : self.auditor_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportAuditCopyCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<AssetReportAuditCopyCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
