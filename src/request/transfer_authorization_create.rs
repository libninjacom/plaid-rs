use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferAuthorizationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub beacon_session_id: Option<String>,
    pub device: Option<TransferAuthorizationDevice>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub network: String,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub payment_profile_token: Option<String>,
    pub type_: String,
    pub user: TransferAuthorizationUserInRequest,
    pub user_present: Option<bool>,
    pub with_guarantee: Option<bool>,
}
impl<'a> TransferAuthorizationCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferAuthorizationCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/transfer/authorization/create");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.beacon_session_id {
            r = r.json(json!({ "beacon_session_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.device {
            r = r.json(json!({ "device" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        r = r.json(json!({ "network" : self.network }));
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.originator_client_id {
            r = r.json(json!({ "originator_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_token {
            r = r.json(json!({ "payment_profile_token" : unwrapped }));
        }
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.with_guarantee {
            r = r.json(json!({ "with_guarantee" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn beacon_session_id(mut self, beacon_session_id: &str) -> Self {
        self.beacon_session_id = Some(beacon_session_id.to_owned());
        self
    }
    pub fn device(mut self, device: TransferAuthorizationDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn with_guarantee(mut self, with_guarantee: bool) -> Self {
        self.with_guarantee = Some(with_guarantee);
        self
    }
}
pub struct TransferAuthorizationCreateRequired<'a> {
    pub amount: &'a str,
    pub network: &'a str,
    pub type_: &'a str,
    pub user: TransferAuthorizationUserInRequest,
}
impl<'a> TransferAuthorizationCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferAuthorizationCreateRequest<'a> {
    type Output = crate::Result<TransferAuthorizationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
