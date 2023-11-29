use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditPayrollIncomePrecheckRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_tokens: Option<Vec<String>>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user_token: Option<String>,
}
impl<'a> CreditPayrollIncomePrecheckRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditPayrollIncomePrecheckResponse> {
        let mut r = self
            .http_client
            .client
            .post("/credit/payroll_income/precheck");
        if let Some(ref unwrapped) = self.access_tokens {
            r = r.json(json!({ "access_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.employer {
            r = r.json(json!({ "employer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payroll_institution {
            r = r.json(json!({ "payroll_institution" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.us_military_info {
            r = r.json(json!({ "us_military_info" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.access_tokens = Some(
            access_tokens
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
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
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.us_military_info = Some(us_military_info);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditPayrollIncomePrecheckRequest<'a> {
    type Output = crate::Result<CreditPayrollIncomePrecheckResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
