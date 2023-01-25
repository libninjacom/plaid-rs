use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditBankIncomeRefreshRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub user_token: String,
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
}
impl<'a> CreditBankIncomeRefreshRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreditBankIncomeRefreshResponse> {
        let mut r = self.http_client.client.post("/credit/bank_income/refresh");
        r = r.json(json!({ "user_token" : self.user_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: CreditBankIncomeRefreshRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditBankIncomeRefreshRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditBankIncomeRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}