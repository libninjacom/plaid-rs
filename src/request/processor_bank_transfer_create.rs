use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_bank_transfer_create`].

On request success, this will return a [`ProcessorBankTransferCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateRequest {
    pub ach_class: Option<String>,
    pub amount: String,
    pub custom_tag: Option<String>,
    pub description: String,
    pub idempotency_key: String,
    pub iso_currency_code: String,
    pub metadata: Option<BankTransferMetadata>,
    pub network: String,
    pub origination_account_id: Option<String>,
    pub processor_token: String,
    pub type_: String,
    pub user: BankTransferUser,
}
impl ProcessorBankTransferCreateRequest {}
pub struct ProcessorBankTransferCreateRequired<'a> {
    pub amount: &'a str,
    pub description: &'a str,
    pub idempotency_key: &'a str,
    pub iso_currency_code: &'a str,
    pub network: &'a str,
    pub processor_token: &'a str,
    pub type_: &'a str,
    pub user: BankTransferUser,
}
impl<'a> ProcessorBankTransferCreateRequired<'a> {}
impl FluentRequest<'_, ProcessorBankTransferCreateRequest> {
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.params.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn custom_tag(mut self, custom_tag: &str) -> Self {
        self.params.custom_tag = Some(custom_tag.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: BankTransferMetadata) -> Self {
        self.params.metadata = Some(metadata);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorBankTransferCreateRequest> {
    type Output = httpclient::InMemoryResult<ProcessorBankTransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/bank_transfer/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(json!({ "ach_class" : unwrapped }));
            }
            r = r.json(json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.custom_tag {
                r = r.json(json!({ "custom_tag" : unwrapped }));
            }
            r = r.json(json!({ "description" : self.params.description }));
            r = r.json(json!({ "idempotency_key" : self.params.idempotency_key }));
            r = r.json(json!({ "iso_currency_code" : self.params.iso_currency_code }));
            if let Some(ref unwrapped) = self.params.metadata {
                r = r.json(json!({ "metadata" : unwrapped }));
            }
            r = r.json(json!({ "network" : self.params.network }));
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(json!({ "origination_account_id" : unwrapped }));
            }
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = r.json(json!({ "type" : self.params.type_ }));
            r = r.json(json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}