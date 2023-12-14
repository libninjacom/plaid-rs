use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_payroll_income_risk_signals_get`].

On request success, this will return a [`CreditPayrollIncomeRiskSignalsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetRequest {
    pub user_token: Option<String>,
}
impl CreditPayrollIncomeRiskSignalsGetRequest {}
impl FluentRequest<'_, CreditPayrollIncomeRiskSignalsGetRequest> {
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomeRiskSignalsGetRequest> {
    type Output = httpclient::InMemoryResult<CreditPayrollIncomeRiskSignalsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/payroll_income/risk_signals/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}