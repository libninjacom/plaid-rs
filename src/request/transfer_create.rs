use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub idempotency_key: Option<String>,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub authorization_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub user: TransferUserInRequest,
    pub metadata: Option<TransferMetadata>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub payment_profile_id: Option<String>,
}
impl<'a> TransferCreateRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TransferCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/create");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        r = r.json(json!({ "authorization_id" : self.authorization_id }));
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "network" : self.network }));
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "description" : self.description }));
        r = r.json(json!({ "ach_class" : self.ach_class }));
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_id {
            r = r.json(json!({ "payment_profile_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.metadata = Some(metadata);
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
    pub fn payment_profile_id(mut self, payment_profile_id: &str) -> Self {
        self.payment_profile_id = Some(payment_profile_id.to_owned());
        self
    }
}
pub struct TransferCreateRequired<'a> {
    pub authorization_id: &'a str,
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub ach_class: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
