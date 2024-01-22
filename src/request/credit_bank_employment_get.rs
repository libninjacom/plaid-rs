use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_bank_employment_get`].

On request success, this will return a [`CreditBankEmploymentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmploymentGetRequest {
    pub user_token: String,
}
impl CreditBankEmploymentGetRequest {}
impl FluentRequest<'_, CreditBankEmploymentGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankEmploymentGetRequest> {
    type Output = httpclient::InMemoryResult<CreditBankEmploymentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/credit/v1/bank_employment/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}