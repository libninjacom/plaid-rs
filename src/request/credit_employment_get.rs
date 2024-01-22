use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_employment_get`].

On request success, this will return a [`CreditEmploymentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditEmploymentGetRequest {
    pub user_token: String,
}
impl CreditEmploymentGetRequest {}
impl FluentRequest<'_, CreditEmploymentGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditEmploymentGetRequest> {
    type Output = httpclient::InMemoryResult<CreditEmploymentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/employment/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}