use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::income_verification_create`].

On request success, this will return a [`IncomeVerificationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequest {
    pub options: Option<IncomeVerificationCreateRequestOptions>,
    pub precheck_id: Option<String>,
    pub webhook: String,
}
impl IncomeVerificationCreateRequest {}
impl FluentRequest<'_, IncomeVerificationCreateRequest> {
    pub fn options(mut self, options: IncomeVerificationCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn precheck_id(mut self, precheck_id: &str) -> Self {
        self.params.precheck_id = Some(precheck_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationCreateRequest> {
    type Output = httpclient::InMemoryResult<IncomeVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/income/verification/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}