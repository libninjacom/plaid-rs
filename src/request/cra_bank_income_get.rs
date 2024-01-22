use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::cra_bank_income_get`].

On request success, this will return a [`CraBankIncomeGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBankIncomeGetRequest {
    pub user_token: Option<String>,
}
impl CraBankIncomeGetRequest {}
impl FluentRequest<'_, CraBankIncomeGetRequest> {
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraBankIncomeGetRequest> {
    type Output = httpclient::InMemoryResult<CraBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/bank_income/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(json!({ "user_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}