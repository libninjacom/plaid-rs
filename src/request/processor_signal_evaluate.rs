use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalEvaluateRequest {
    pub amount: f64,
    pub client_transaction_id: String,
    pub client_user_id: Option<String>,
    pub default_payment_method: Option<String>,
    pub device: Option<SignalDevice>,
    pub is_recurring: Option<bool>,
    pub processor_token: String,
    pub user: Option<SignalUser>,
    pub user_present: Option<bool>,
}
impl ProcessorSignalEvaluateRequest {}
impl FluentRequest<'_, ProcessorSignalEvaluateRequest> {
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalEvaluateRequest> {
    type Output = httpclient::InMemoryResult<ProcessorSignalEvaluateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/signal/evaluate";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}