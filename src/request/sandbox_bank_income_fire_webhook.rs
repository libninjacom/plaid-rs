use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_bank_income_fire_webhook`].

On request success, this will return a [`SandboxBankIncomeFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankIncomeFireWebhookRequest {
    pub webhook_code: String,
    pub webhook_fields: SandboxBankIncomeWebhookFireRequestWebhookFields,
    pub webhook_override: Option<String>,
}
impl SandboxBankIncomeFireWebhookRequest {}
impl FluentRequest<'_, SandboxBankIncomeFireWebhookRequest> {
    pub fn webhook_override(mut self, webhook_override: &str) -> Self {
        self.params.webhook_override = Some(webhook_override.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankIncomeFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<SandboxBankIncomeFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/bank_income/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}