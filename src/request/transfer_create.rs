use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_create`].

On request success, this will return a [`TransferCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<String>,
    pub amount: Option<String>,
    pub authorization_id: String,
    pub description: String,
    pub facilitator_fee: Option<String>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub network: Option<String>,
    pub origination_account_id: Option<String>,
    pub test_clock_id: Option<String>,
    pub type_: Option<String>,
    pub user: Option<TransferUserInRequestDeprecated>,
}
impl TransferCreateRequest {}
pub struct TransferCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub authorization_id: &'a str,
    pub description: &'a str,
}
impl<'a> TransferCreateRequired<'a> {}
impl FluentRequest<'_, TransferCreateRequest> {
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.params.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn amount(mut self, amount: &str) -> Self {
        self.params.amount = Some(amount.to_owned());
        self
    }
    pub fn facilitator_fee(mut self, facilitator_fee: &str) -> Self {
        self.params.facilitator_fee = Some(facilitator_fee.to_owned());
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
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.params.metadata = Some(metadata);
        self
    }
    pub fn network(mut self, network: &str) -> Self {
        self.params.network = Some(network.to_owned());
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
    pub fn user(mut self, user: TransferUserInRequestDeprecated) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(json!({ "ach_class" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.amount {
                r = r.json(json!({ "amount" : unwrapped }));
            }
            r = r.json(json!({ "authorization_id" : self.params.authorization_id }));
            r = r.json(json!({ "description" : self.params.description }));
            if let Some(ref unwrapped) = self.params.facilitator_fee {
                r = r.json(json!({ "facilitator_fee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.idempotency_key {
                r = r.json(json!({ "idempotency_key" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(json!({ "iso_currency_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.metadata {
                r = r.json(json!({ "metadata" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.network {
                r = r.json(json!({ "network" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(json!({ "origination_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(json!({ "test_clock_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(json!({ "type" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}