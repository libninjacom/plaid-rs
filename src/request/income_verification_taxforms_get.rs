use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IncomeVerificationTaxformsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationTaxformsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<IncomeVerificationTaxformsGetResponse> {
        let mut r = self.http_client.client.post("/income/verification/taxforms/get");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for IncomeVerificationTaxformsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<IncomeVerificationTaxformsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
