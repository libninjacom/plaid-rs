use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditBankIncomeGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub user_token: Option<String>,
    pub options: Option<CreditBankIncomeGetRequestOptions>,
}
impl<'a> CreditBankIncomeGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreditBankIncomeGetResponse> {
        let mut r = self.http_client.client.post("/credit/bank_income/get");
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
    pub fn options(mut self, options: CreditBankIncomeGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditBankIncomeGetRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}