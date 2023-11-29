use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorBankTransferCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
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
impl<'a> ProcessorBankTransferCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<ProcessorBankTransferCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/processor/bank_transfer/create");
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.custom_tag {
            r = r.json(json!({ "custom_tag" : unwrapped }));
        }
        r = r.json(json!({ "description" : self.description }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "iso_currency_code" : self.iso_currency_code }));
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        r = r.json(json!({ "network" : self.network }));
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = r.json(json!({ "type" : self.type_ }));
        r = r.json(json!({ "user" : self.user }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn custom_tag(mut self, custom_tag: &str) -> Self {
        self.custom_tag = Some(custom_tag.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: BankTransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
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
impl<'a> ::std::future::IntoFuture for ProcessorBankTransferCreateRequest<'a> {
    type Output = crate::Result<ProcessorBankTransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
