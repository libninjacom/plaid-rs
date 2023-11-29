use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IncomeVerificationPrecheckRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub transactions_access_token: Option<String>,
    pub transactions_access_tokens: Option<Vec<String>>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user: Option<IncomeVerificationPrecheckUser>,
}
impl<'a> IncomeVerificationPrecheckRequest<'a> {
    pub async fn send(self) -> crate::Result<IncomeVerificationPrecheckResponse> {
        let mut r = self
            .http_client
            .client
            .post("/income/verification/precheck");
        if let Some(ref unwrapped) = self.employer {
            r = r.json(json!({ "employer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payroll_institution {
            r = r.json(json!({ "payroll_institution" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transactions_access_token {
            r = r.json(json!({ "transactions_access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transactions_access_tokens {
            r = r.json(json!({ "transactions_access_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.us_military_info {
            r = r.json(json!({ "us_military_info" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn employer(mut self, employer: IncomeVerificationPrecheckEmployer) -> Self {
        self.employer = Some(employer);
        self
    }
    pub fn payroll_institution(
        mut self,
        payroll_institution: IncomeVerificationPrecheckPayrollInstitution,
    ) -> Self {
        self.payroll_institution = Some(payroll_institution);
        self
    }
    pub fn transactions_access_token(mut self, transactions_access_token: &str) -> Self {
        self.transactions_access_token = Some(transactions_access_token.to_owned());
        self
    }
    pub fn transactions_access_tokens(
        mut self,
        transactions_access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.transactions_access_tokens = Some(
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
        self.us_military_info = Some(us_military_info);
        self
    }
    pub fn user(mut self, user: IncomeVerificationPrecheckUser) -> Self {
        self.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for IncomeVerificationPrecheckRequest<'a> {
    type Output = crate::Result<IncomeVerificationPrecheckResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
