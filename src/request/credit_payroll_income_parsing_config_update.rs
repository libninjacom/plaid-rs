use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_payroll_income_parsing_config_update`].

On request success, this will return a [`CreditPayrollIncomeParsingConfigUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeParsingConfigUpdateRequest {
    pub item_id: Option<String>,
    pub parsing_config: Vec<String>,
    pub user_token: String,
}
impl CreditPayrollIncomeParsingConfigUpdateRequest {}
impl FluentRequest<'_, CreditPayrollIncomeParsingConfigUpdateRequest> {
    pub fn item_id(mut self, item_id: &str) -> Self {
        self.params.item_id = Some(item_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomeParsingConfigUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        CreditPayrollIncomeParsingConfigUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/payroll_income/parsing_config/update";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}