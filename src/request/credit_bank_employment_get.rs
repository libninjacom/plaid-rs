use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreditBankEmploymentGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditBankEmploymentGetRequest<'a> {
    pub async fn send(self) -> crate::Result<CreditBankEmploymentGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/beta/credit/v1/bank_employment/get");
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for CreditBankEmploymentGetRequest<'a> {
    type Output = crate::Result<CreditBankEmploymentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
