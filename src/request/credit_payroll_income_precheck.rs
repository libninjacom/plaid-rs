use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckRequest {
    pub access_tokens: Option<Vec<String>>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user_token: Option<String>,
}
impl CreditPayrollIncomePrecheckRequest {}
impl FluentRequest<'_, CreditPayrollIncomePrecheckRequest> {
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn employer(mut self, employer: IncomeVerificationPrecheckEmployer) -> Self {
        self.params.employer = Some(employer);
        self
    }
    pub fn payroll_institution(
        mut self,
        payroll_institution: IncomeVerificationPrecheckPayrollInstitution,
    ) -> Self {
        self.params.payroll_institution = Some(payroll_institution);
        self
    }
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.params.us_military_info = Some(us_military_info);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomePrecheckRequest> {
    type Output = httpclient::InMemoryResult<CreditPayrollIncomePrecheckResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/payroll_income/precheck";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}