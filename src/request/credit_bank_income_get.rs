use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditBankIncomeGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<CreditBankIncomeGetRequestOptions>,
    pub user_token: Option<String>,
}
impl<'a> CreditBankIncomeGetRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditBankIncomeGetResponse> {
        let mut r = self.http_client.client.post("/credit/bank_income/get");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: CreditBankIncomeGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreditBankIncomeGetRequest<'a> {
    type Output = crate::Result<CreditBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
