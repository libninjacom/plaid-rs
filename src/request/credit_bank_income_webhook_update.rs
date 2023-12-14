use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_bank_income_webhook_update`].

On request success, this will return a [`CreditBankIncomeWebhookUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeWebhookUpdateRequest {
    pub enable_webhooks: bool,
    pub user_token: String,
}
impl CreditBankIncomeWebhookUpdateRequest {}
impl FluentRequest<'_, CreditBankIncomeWebhookUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankIncomeWebhookUpdateRequest> {
    type Output = httpclient::InMemoryResult<CreditBankIncomeWebhookUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/bank_income/webhook/update";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}