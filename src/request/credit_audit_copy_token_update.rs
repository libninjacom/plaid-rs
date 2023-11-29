use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditAuditCopyTokenUpdateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub audit_copy_token: String,
    pub report_tokens: Vec<String>,
}
impl<'a> CreditAuditCopyTokenUpdateRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditAuditCopyTokenUpdateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/credit/audit_copy_token/update");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = r.json(json!({ "report_tokens" : self.report_tokens }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for CreditAuditCopyTokenUpdateRequest<'a> {
    type Output = crate::Result<CreditAuditCopyTokenUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
