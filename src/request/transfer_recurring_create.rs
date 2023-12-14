use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_recurring_create`].

On request success, this will return a [`TransferRecurringCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub description: String,
    pub device: Option<TransferDevice>,
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
impl TransferRecurringCreateRequest {}
pub struct TransferRecurringCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub idempotency_key: &'a str,
    pub network: &'a str,
    pub schedule: TransferRecurringSchedule,
    pub type_: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferRecurringCreateRequired<'a> {}
impl FluentRequest<'_, TransferRecurringCreateRequest> {
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.params.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn device(mut self, device: TransferDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.params.user_present = Some(user_present);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRecurringCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferRecurringCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/recurring/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}