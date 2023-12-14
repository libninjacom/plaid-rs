use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequest {
    pub options: Option<CreditBankIncomeGetRequestOptions>,
    pub user_token: Option<String>,
}
impl CreditBankIncomeGetRequest {}
impl FluentRequest<'_, CreditBankIncomeGetRequest> {
    pub fn options(mut self, options: CreditBankIncomeGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditBankIncomeGetRequest> {
    type Output = httpclient::InMemoryResult<CreditBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/bank_income/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}