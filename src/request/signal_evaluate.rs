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
    pub risk_profile_key: Option<String>,
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
    pub fn risk_profile_key(mut self, risk_profile_key: &str) -> Self {
        self.params.risk_profile_key = Some(risk_profile_key.to_owned());
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
        Box::pin(async move {
            let url = "/signal/evaluate";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_id" : self.params.account_id }));
            r = r.json(json!({ "amount" : self.params.amount }));
            r = r
                .json(
                    json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.default_payment_method {
                r = r.json(json!({ "default_payment_method" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.device {
                r = r.json(json!({ "device" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.is_recurring {
                r = r.json(json!({ "is_recurring" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.risk_profile_key {
                r = r.json(json!({ "risk_profile_key" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(json!({ "user" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_present {
                r = r.json(json!({ "user_present" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}