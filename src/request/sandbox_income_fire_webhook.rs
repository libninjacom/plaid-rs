use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_income_fire_webhook`].

On request success, this will return a [`SandboxIncomeFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookRequest {
    pub item_id: String,
    pub user_id: Option<String>,
    pub verification_status: Option<String>,
    pub webhook: String,
    pub webhook_code: String,
}
impl SandboxIncomeFireWebhookRequest {}
impl FluentRequest<'_, SandboxIncomeFireWebhookRequest> {
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.params.user_id = Some(user_id.to_owned());
        self
    }
    pub fn verification_status(mut self, verification_status: &str) -> Self {
        self.params.verification_status = Some(verification_status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxIncomeFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<SandboxIncomeFireWebhookResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/income/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}