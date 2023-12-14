use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::income_verification_taxforms_get`].

On request success, this will return a [`IncomeVerificationTaxformsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetRequest {
    pub access_token: Option<String>,
    pub income_verification_id: Option<String>,
}
impl IncomeVerificationTaxformsGetRequest {}
impl FluentRequest<'_, IncomeVerificationTaxformsGetRequest> {
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.params.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationTaxformsGetRequest> {
    type Output = httpclient::InMemoryResult<IncomeVerificationTaxformsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/income/verification/taxforms/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}