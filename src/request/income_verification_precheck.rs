use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::income_verification_precheck`].

On request success, this will return a [`IncomeVerificationPrecheckResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub transactions_access_token: Option<String>,
    pub transactions_access_tokens: Option<Vec<String>>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user: Option<IncomeVerificationPrecheckUser>,
}
impl IncomeVerificationPrecheckRequest {}
impl FluentRequest<'_, IncomeVerificationPrecheckRequest> {
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
    pub fn transactions_access_token(mut self, transactions_access_token: &str) -> Self {
        self
            .params
            .transactions_access_token = Some(transactions_access_token.to_owned());
        self
    }
    pub fn transactions_access_tokens(
        mut self,
        transactions_access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .transactions_access_tokens = Some(
            transactions_access_tokens
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.params.us_military_info = Some(us_military_info);
        self
    }
    pub fn user(mut self, user: IncomeVerificationPrecheckUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationPrecheckRequest> {
    type Output = httpclient::InMemoryResult<IncomeVerificationPrecheckResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/precheck";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.employer {
                r = r.json(json!({ "employer" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payroll_institution {
                r = r.json(json!({ "payroll_institution" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transactions_access_token {
                r = r.json(json!({ "transactions_access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transactions_access_tokens {
                r = r.json(json!({ "transactions_access_tokens" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.us_military_info {
                r = r.json(json!({ "us_military_info" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}