use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferIntentCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub account_id: Option<String>,
    pub ach_class: Option<String>,
    pub amount: String,
    pub description: String,
    pub funding_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub mode: String,
    pub network: Option<String>,
    pub origination_account_id: Option<String>,
    pub require_guarantee: Option<bool>,
    pub user: TransferUserInRequest,
}
impl<'a> TransferIntentCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferIntentCreateResponse> {
        let mut r = self.http_client.client.post("/transfer/intent/create");
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ach_class {
            r = r.json(json!({ "ach_class" : unwrapped }));
        }
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "description" : self.description }));
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.json(json!({ "metadata" : unwrapped }));
        }
        r = r.json(json!({ "mode" : self.mode }));
        if let Some(ref unwrapped) = self.network {
            r = r.json(json!({ "network" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.require_guarantee {
            r = r.json(json!({ "require_guarantee" : unwrapped }));
        }
        r = r.json(json!({ "user" : self.user }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
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
    pub fn require_guarantee(mut self, require_guarantee: bool) -> Self {
        self.require_guarantee = Some(require_guarantee);
        self
    }
}
pub struct TransferIntentCreateRequired<'a> {
    pub amount: &'a str,
    pub description: &'a str,
    pub mode: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferIntentCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for TransferIntentCreateRequest<'a> {
    type Output = crate::Result<TransferIntentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
