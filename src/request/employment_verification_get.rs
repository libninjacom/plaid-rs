use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::employment_verification_get`].

On request success, this will return a [`EmploymentVerificationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentVerificationGetRequest {
    pub access_token: String,
}
impl EmploymentVerificationGetRequest {}
impl FluentRequest<'_, EmploymentVerificationGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, EmploymentVerificationGetRequest> {
    type Output = httpclient::InMemoryResult<EmploymentVerificationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/employment/verification/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}