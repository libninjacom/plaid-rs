use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerDepositRequest {
    pub amount: String,
    pub description: Option<String>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: String,
    pub network: String,
    pub originator_client_id: Option<String>,
}
impl TransferLedgerDepositRequest {}
impl FluentRequest<'_, TransferLedgerDepositRequest> {
    pub fn description(mut self, description: &str) -> Self {
        self.params.description = Some(description.to_owned());
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferLedgerDepositRequest> {
    type Output = httpclient::InMemoryResult<TransferLedgerDepositResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/ledger/deposit";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}