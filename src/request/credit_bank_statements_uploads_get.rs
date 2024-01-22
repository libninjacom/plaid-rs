use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_bank_statements_uploads_get`].

On request success, this will return a [`CreditBankStatementsUploadsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankStatementsUploadsGetRequest {
    pub user_token: String,
}
impl CreditBankStatementsUploadsGetRequest {}
impl FluentRequest<'_, CreditBankStatementsUploadsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankStatementsUploadsGetRequest> {
    type Output = httpclient::InMemoryResult<CreditBankStatementsUploadsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_statements/uploads/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}