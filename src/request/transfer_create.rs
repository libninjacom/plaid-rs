use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: Option<String>,
    pub authorization_id: String,
    pub description: String,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub network: Option<String>,
    pub origination_account_id: Option<String>,
    pub payment_profile_token: Option<String>,
    pub type_: Option<String>,
    pub user: Option<TransferUserInRequestDeprecated>,
}
impl<'a> TransferCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/create");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amount {
            r = r.json(json!({ "amount" : unwrapped }));
        }
        r = r.json(json!({ "authorization_id" : self.authorization_id }));
        r = r.json(json!({ "description" : self.description }));
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.network {
            r = r.json(json!({ "network" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_token {
            r = r.json(json!({ "payment_profile_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.json(json!({ "type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
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
    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.to_owned());
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
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn network(mut self, network: &str) -> Self {
        self.network = Some(network.to_owned());
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
    pub fn user(mut self, user: TransferUserInRequestDeprecated) -> Self {
        self.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferCreateRequest<'a> {
    type Output = crate::Result<TransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
