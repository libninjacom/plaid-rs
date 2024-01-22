use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_report_audit_copy_remove`].

On request success, this will return a [`CreditAuditCopyTokenRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditReportAuditCopyRemoveRequest {
    pub audit_copy_token: String,
}
impl CreditReportAuditCopyRemoveRequest {}
impl FluentRequest<'_, CreditReportAuditCopyRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditReportAuditCopyRemoveRequest> {
    type Output = httpclient::InMemoryResult<CreditAuditCopyTokenRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/audit_copy_token/remove";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "audit_copy_token" : self.params.audit_copy_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}