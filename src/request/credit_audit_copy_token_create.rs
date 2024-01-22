use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_audit_copy_token_create`].

On request success, this will return a [`CreditAuditCopyTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateRequest {
    pub report_tokens: Vec<String>,
}
impl CreditAuditCopyTokenCreateRequest {}
impl FluentRequest<'_, CreditAuditCopyTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAuditCopyTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<CreditAuditCopyTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/audit_copy_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "report_tokens" : self.params.report_tokens }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}