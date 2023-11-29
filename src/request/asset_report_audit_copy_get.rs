use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AssetReportAuditCopyGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyGetRequest<'a> {
    pub async fn send(self) -> crate::Result<AssetReportGetResponse> {
        let mut r = self.http_client.client.post("/asset_report/audit_copy/get");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for AssetReportAuditCopyGetRequest<'a> {
    type Output = crate::Result<AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
