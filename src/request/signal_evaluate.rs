use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SignalEvaluateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
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
impl<'a> SignalEvaluateRequest<'a> {
    pub async fn send(self) -> crate::Result<SignalEvaluateResponse> {
        let mut r = self.http_client.client.post("/signal/evaluate");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_id" : self.account_id }));
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "client_transaction_id" : self.client_transaction_id }));
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default_payment_method {
            r = r.json(json!({ "default_payment_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.device {
            r = r.json(json!({ "device" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_recurring {
            r = r.json(json!({ "is_recurring" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn default_payment_method(mut self, default_payment_method: &str) -> Self {
        self.default_payment_method = Some(default_payment_method.to_owned());
        self
    }
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn is_recurring(mut self, is_recurring: bool) -> Self {
        self.is_recurring = Some(is_recurring);
        self
    }
    pub fn user(mut self, user: SignalUser) -> Self {
        self.user = Some(user);
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
}
pub struct SignalEvaluateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: f64,
    pub client_transaction_id: &'a str,
}
impl<'a> SignalEvaluateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for SignalEvaluateRequest<'a> {
    type Output = crate::Result<SignalEvaluateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
