use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::accounts_balance_get`].

On request success, this will return a [`AccountsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    pub access_token: String,
    pub options: Option<AccountsBalanceGetRequestOptions>,
    pub payment_details: Option<AccountsBalanceGetRequestPaymentDetails>,
}
impl AccountsBalanceGetRequest {}
impl FluentRequest<'_, AccountsBalanceGetRequest> {
    pub fn options(mut self, options: AccountsBalanceGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn payment_details(
        mut self,
        payment_details: AccountsBalanceGetRequestPaymentDetails,
    ) -> Self {
        self.params.payment_details = Some(payment_details);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AccountsBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<AccountsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/accounts/balance/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payment_details {
                r = r.json(json!({ "payment_details" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}