use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_audit_copy_token_update`].

On request success, this will return a [`CreditAuditCopyTokenUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenUpdateRequest {
    pub audit_copy_token: String,
    pub report_tokens: Vec<String>,
}
impl CreditAuditCopyTokenUpdateRequest {}
impl FluentRequest<'_, CreditAuditCopyTokenUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAuditCopyTokenUpdateRequest> {
    type Output = httpclient::InMemoryResult<CreditAuditCopyTokenUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/audit_copy_token/update";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "audit_copy_token" : self.params.audit_copy_token }));
            r = r.json(json!({ "report_tokens" : self.params.report_tokens }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}