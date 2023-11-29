use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportAuditCopyRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyRemoveRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportAuditCopyRemoveResponse> {
        let mut r = self
            .http_client
            .client
            .post("/asset_report/audit_copy/remove");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportAuditCopyRemoveRequest<'a> {
    type Output = crate::Result<AssetReportAuditCopyRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
