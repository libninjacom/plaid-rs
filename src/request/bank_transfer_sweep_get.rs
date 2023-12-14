use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_sweep_get`].

On request success, this will return a [`BankTransferSweepGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferSweepGetRequest {
    pub sweep_id: String,
}
impl BankTransferSweepGetRequest {}
impl FluentRequest<'_, BankTransferSweepGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferSweepGetRequest> {
    type Output = httpclient::InMemoryResult<BankTransferSweepGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/bank_transfer/sweep/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}