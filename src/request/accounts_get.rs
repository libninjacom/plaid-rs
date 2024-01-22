use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::accounts_get`].

On request success, this will return a [`AccountsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    pub access_token: String,
    pub options: Option<AccountsGetRequestOptions>,
}
impl AccountsGetRequest {}
impl FluentRequest<'_, AccountsGetRequest> {
    pub fn options(mut self, options: AccountsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AccountsGetRequest> {
    type Output = httpclient::InMemoryResult<AccountsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/accounts/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}