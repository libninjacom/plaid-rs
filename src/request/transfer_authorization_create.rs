use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferAuthorizationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub ach_class: String,
    pub user: TransferAuthorizationUserInRequest,
    pub device: Option<TransferAuthorizationDevice>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub user_present: Option<bool>,
    pub payment_profile_id: Option<String>,
}
impl<'a> TransferAuthorizationCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransferAuthorizationCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/authorization/create");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "network" : self.network }));
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "ach_class" : self.ach_class }));
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.device {
            r = r.json(json!({ "device" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_present {
            r = r.json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_id {
            r = r.json(json!({ "payment_profile_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn device(mut self, device: TransferAuthorizationDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn payment_profile_id(mut self, payment_profile_id: &str) -> Self {
        self.payment_profile_id = Some(payment_profile_id.to_owned());
        self
    }
}
pub struct TransferAuthorizationCreateRequired<'a> {
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub ach_class: &'a str,
    pub user: TransferAuthorizationUserInRequest,
}
impl<'a> TransferAuthorizationCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferAuthorizationCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferAuthorizationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
