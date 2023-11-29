use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRecurringCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub description: String,
    pub device: TransferDevice,
    pub funding_account_id: Option<String>,
    pub idempotency_key: String,
    pub iso_currency_code: Option<String>,
    pub network: String,
    pub schedule: TransferRecurringSchedule,
    pub test_clock_id: Option<String>,
    pub type_: String,
    pub user: TransferUserInRequest,
    pub user_present: Option<bool>,
}
impl<'a> TransferRecurringCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferRecurringCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/recurring/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_id" : self.account_id }));
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "description" : self.description }));
        r = r.json(json!({ "device" : self.device }));
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        r = r.json(json!({ "network" : self.network }));
        r = r.json(json!({ "schedule" : self.schedule }));
        if let Some(ref unwrapped) = self.test_clock_id {
            r = r.json(json!({ "test_clock_id" : unwrapped }));
        }
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.funding_account_id = Some(funding_account_id.to_owned());
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
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
}
pub struct TransferRecurringCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub device: TransferDevice,
    pub idempotency_key: &'a str,
    pub network: &'a str,
    pub schedule: TransferRecurringSchedule,
    pub type_: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferRecurringCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferRecurringCreateRequest<'a> {
    type Output = crate::Result<TransferRecurringCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
