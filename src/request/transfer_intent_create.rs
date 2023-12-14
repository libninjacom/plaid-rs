use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_intent_create`].

On request success, this will return a [`TransferIntentCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
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
impl TransferIntentCreateRequest {}
pub struct TransferIntentCreateRequired<'a> {
    pub amount: &'a str,
    pub description: &'a str,
    pub mode: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferIntentCreateRequired<'a> {}
impl FluentRequest<'_, TransferIntentCreateRequest> {
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.params.account_id = Some(account_id.to_owned());
        self
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.params.ach_class = Some(ach_class.to_owned());
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
    pub fn require_guarantee(mut self, require_guarantee: bool) -> Self {
        self.params.require_guarantee = Some(require_guarantee);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferIntentCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferIntentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/intent/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}