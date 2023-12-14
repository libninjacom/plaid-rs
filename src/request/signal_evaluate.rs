use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::signal_evaluate`].

On request success, this will return a [`SignalEvaluateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvaluateRequest {
    pub access_token: String,
    pub account_id: String,
    pub amount: f64,
    pub client_transaction_id: String,
    pub client_user_id: Option<String>,
    pub default_payment_method: Option<String>,
    pub device: Option<SignalDevice>,
    pub is_recurring: Option<bool>,
    pub user: Option<SignalUser>,
    pub user_present: Option<bool>,
}
impl SignalEvaluateRequest {}
pub struct SignalEvaluateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: f64,
    pub client_transaction_id: &'a str,
}
impl<'a> SignalEvaluateRequired<'a> {}
impl FluentRequest<'_, SignalEvaluateRequest> {
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn default_payment_method(mut self, default_payment_method: &str) -> Self {
        self.params.default_payment_method = Some(default_payment_method.to_owned());
        self
    }
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    pub fn is_recurring(mut self, is_recurring: bool) -> Self {
        self.params.is_recurring = Some(is_recurring);
        self
    }
    pub fn user(mut self, user: SignalUser) -> Self {
        self.params.user = Some(user);
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.params.user_present = Some(user_present);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalEvaluateRequest> {
    type Output = httpclient::InMemoryResult<SignalEvaluateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/signal/evaluate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}