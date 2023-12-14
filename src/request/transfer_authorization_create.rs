use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_authorization_create`].

On request success, this will return a [`TransferAuthorizationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: String,
    pub beacon_session_id: Option<String>,
    pub credit_funds_source: Option<TransferCreditFundsSource>,
    pub device: Option<TransferAuthorizationDevice>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub network: String,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub payment_profile_token: Option<String>,
    pub test_clock_id: Option<String>,
    pub type_: String,
    pub user: TransferAuthorizationUserInRequest,
    pub user_present: Option<bool>,
    pub with_guarantee: Option<bool>,
}
impl TransferAuthorizationCreateRequest {}
pub struct TransferAuthorizationCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub network: &'a str,
    pub type_: &'a str,
    pub user: TransferAuthorizationUserInRequest,
}
impl<'a> TransferAuthorizationCreateRequired<'a> {}
impl FluentRequest<'_, TransferAuthorizationCreateRequest> {
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.params.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn beacon_session_id(mut self, beacon_session_id: &str) -> Self {
        self.params.beacon_session_id = Some(beacon_session_id.to_owned());
        self
    }
    pub fn credit_funds_source(
        mut self,
        credit_funds_source: TransferCreditFundsSource,
    ) -> Self {
        self.params.credit_funds_source = Some(credit_funds_source);
        self
    }
    pub fn device(mut self, device: TransferAuthorizationDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.params.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.params.payment_profile_token = Some(payment_profile_token.to_owned());
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
    pub fn with_guarantee(mut self, with_guarantee: bool) -> Self {
        self.params.with_guarantee = Some(with_guarantee);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferAuthorizationCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferAuthorizationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/authorization/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}