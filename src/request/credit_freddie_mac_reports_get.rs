use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_freddie_mac_reports_get`].

On request success, this will return a [`CreditFreddieMacReportsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetRequest {
    pub audit_copy_token: String,
}
impl CreditFreddieMacReportsGetRequest {}
impl FluentRequest<'_, CreditFreddieMacReportsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditFreddieMacReportsGetRequest> {
    type Output = httpclient::InMemoryResult<CreditFreddieMacReportsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/freddie_mac/reports/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "audit_copy_token" : self.params.audit_copy_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}