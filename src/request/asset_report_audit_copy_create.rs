use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportAuditCopyCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub asset_report_token: String,
    pub auditor_id: Option<String>,
}
impl<'a> AssetReportAuditCopyCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportAuditCopyCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/asset_report/audit_copy/create");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.auditor_id {
            r = r.json(json!({ "auditor_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn auditor_id(mut self, auditor_id: &str) -> Self {
        self.auditor_id = Some(auditor_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportAuditCopyCreateRequest<'a> {
    type Output = crate::Result<AssetReportAuditCopyCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
