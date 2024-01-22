use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_stripe_bank_account_token_create`].

On request success, this will return a [`ProcessorStripeBankAccountTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
}
impl ProcessorStripeBankAccountTokenCreateRequest {}
impl FluentRequest<'_, ProcessorStripeBankAccountTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorStripeBankAccountTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        ProcessorStripeBankAccountTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/stripe/bank_account_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_id" : self.params.account_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}