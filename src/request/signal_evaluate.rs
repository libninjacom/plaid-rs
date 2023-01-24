use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SignalEvaluateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub client_transaction_id: String,
    pub amount: f64,
    pub user_present: Option<bool>,
    pub client_user_id: Option<String>,
    pub user: Option<SignalUser>,
    pub device: Option<SignalDevice>,
}
impl<'a> SignalEvaluateRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SignalEvaluateResponse> {
        let mut r = self.http_client.client.post("/signal/evaluate");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_id" : self.account_id }));
        r = r.json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.device {
            r = r.json(json!({ "device" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn user(mut self, user: SignalUser) -> Self {
        self.user = Some(user);
        self
    }
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.device = Some(device);
        self
    }
}
pub struct SignalEvaluateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub client_transaction_id: &'a str,
    pub amount: f64,
}
impl<'a> SignalEvaluateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for SignalEvaluateRequest<'a> {
    type Output = httpclient::InMemoryResult<SignalEvaluateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
