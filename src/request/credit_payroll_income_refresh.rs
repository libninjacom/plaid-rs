use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditPayrollIncomeRefreshRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<CreditPayrollIncomeRefreshRequestOptions>,
    pub user_token: String,
}
impl<'a> CreditPayrollIncomeRefreshRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreditPayrollIncomeRefreshResponse> {
        let mut r = self.http_client.client.post("/credit/payroll_income/refresh");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: CreditPayrollIncomeRefreshRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditPayrollIncomeRefreshRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditPayrollIncomeRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}