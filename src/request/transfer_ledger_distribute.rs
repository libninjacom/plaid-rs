use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerDistributeRequest {
    pub amount: String,
    pub description: Option<String>,
    pub from_client_id: String,
    pub idempotency_key: String,
    pub to_client_id: String,
}
impl TransferLedgerDistributeRequest {}
pub struct TransferLedgerDistributeRequired<'a> {
    pub amount: &'a str,
    pub from_client_id: &'a str,
    pub idempotency_key: &'a str,
    pub to_client_id: &'a str,
}
impl<'a> TransferLedgerDistributeRequired<'a> {}
impl FluentRequest<'_, TransferLedgerDistributeRequest> {
    pub fn description(mut self, description: &str) -> Self {
        self.params.description = Some(description.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferLedgerDistributeRequest> {
    type Output = httpclient::InMemoryResult<TransferLedgerDistributeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/ledger/distribute";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}