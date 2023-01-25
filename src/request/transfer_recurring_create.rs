use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRecurringCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub idempotency_key: String,
    pub account_id: String,
    pub type_: String,
    pub network: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub user_present: Option<bool>,
    pub iso_currency_code: Option<String>,
    pub description: String,
    pub test_clock_id: Option<String>,
    pub schedule: TransferRecurringSchedule,
    pub user: TransferUserInRequest,
    pub device: TransferDevice,
}
impl<'a> TransferRecurringCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferRecurringCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/recurring/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "account_id" : self.account_id }));
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "network" : self.network }));
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        r = r.json(json!({ "description" : self.description }));
        if let Some(ref unwrapped) = self.test_clock_id {
            r = r.json(json!({ "test_clock_id" : unwrapped }));
        }
        r = r.json(json!({ "schedule" : self.schedule }));
        r = r.json(json!({ "user" : self.user }));
        r = r.json(json!({ "device" : self.device }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
}
pub struct TransferRecurringCreateRequired<'a> {
    pub access_token: &'a str,
    pub idempotency_key: &'a str,
    pub account_id: &'a str,
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub schedule: TransferRecurringSchedule,
    pub user: TransferUserInRequest,
    pub device: TransferDevice,
}
impl<'a> TransferRecurringCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferRecurringCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferRecurringCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}