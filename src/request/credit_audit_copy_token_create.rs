use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditAuditCopyTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub report_tokens: Vec<String>,
}
impl<'a> CreditAuditCopyTokenCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditAuditCopyTokenCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/credit/audit_copy_token/create");
        r = r.json(json!({ "report_tokens" : self.report_tokens }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for CreditAuditCopyTokenCreateRequest<'a> {
    type Output = crate::Result<CreditAuditCopyTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
